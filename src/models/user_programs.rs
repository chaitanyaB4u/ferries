use diesel::prelude::*;

use crate::models::coaches::Coach;
use crate::models::enrollments::Enrollment;
use crate::models::programs::Program;

use crate::schema::coaches::dsl::*;
use crate::schema::enrollments::dsl::*;
use crate::schema::programs;
use crate::schema::programs::dsl::*;

#[derive(juniper::GraphQLEnum)]
pub enum Desire {
    EXPLORE,
    ENROLLED,
    YOURS,
    SINGLE,
}

#[derive(juniper::GraphQLInputObject)]
pub struct ProgramCriteria {
    user_id: String,
    program_id: String,
    desire: Desire,
}

#[derive(juniper::GraphQLEnum)]
pub enum EnrollmentStatus {
    UNKNOWN,
    YES,
    NO,
}

pub struct ProgramRow {
    pub program: Program,
    pub coach: Coach,
    pub enrollment_id: String,
    pub enrollment_status: EnrollmentStatus,
}

#[juniper::object]
impl ProgramRow {
    pub fn program(&self) -> &Program {
        &self.program
    }

    pub fn coach(&self) -> &Coach {
        &self.coach
    }

    pub fn enrollment_status(&self) -> &EnrollmentStatus {
        &self.enrollment_status
    }

    pub fn enrollment_id(&self) -> &str {
        &self.enrollment_id
    }
}

type ProgramType = (Program, Coach);

pub type ProgramResult = Result<Vec<ProgramRow>, diesel::result::Error>;

pub fn get_programs(connection: &MysqlConnection, criteria: &ProgramCriteria) -> ProgramResult {
    match &criteria.desire {
        Desire::EXPLORE => get_latest_programs(connection),
        Desire::ENROLLED => get_enrolled_programs(connection, criteria),
        Desire::YOURS => get_coach_programs(connection, criteria),
        Desire::SINGLE => find_program(connection, criteria),
    }
}

/**
 * The enrollment may be directly in the parent program or in one of the children
 */
fn get_enrollment(connection: &MysqlConnection, criteria: &ProgramCriteria, _parent_program_id: &str) -> Option<Enrollment> {

    let prog_query = programs.filter(parent_program_id.eq(_parent_program_id)).select(programs::id);

    let enrollment_data: QueryResult<Enrollment> = enrollments.filter(member_id.eq(&criteria.user_id)).filter(program_id.eq_any(prog_query)).first(connection);

    match enrollment_data {
        Ok(enrollment) => Some(enrollment),
        Err(_) => None,
    }
}

/**
 * Discovering the Kind of Relationship between the User and the Program.
 * 
 * Discover if the User is associated with this program as a Coach or a Member.
 * 
 * If Enrolled
 *    Return the enrolled program over the criteria program
 * 
 * If one of the coaches of the Program, aka Peer Coach,
 *    Return the associated program of the Coach.
 *   
 * If None of the above
 *    Return allway the Parent Program
 */
fn find_program(connection: &MysqlConnection, criteria: &ProgramCriteria) -> ProgramResult {

    // Grep the Program by the given Id
    let result: (Program, Coach) = programs.inner_join(coaches).filter(programs::id.eq(&criteria.program_id)).first(connection)?;
    let program = result.0;
    let coach = result.1;

    // Check is an enrolled Member
    let enrollment_result = get_enrollment(connection, criteria, program.coalesce_parent_id());
    if let Some(enrollment) = enrollment_result {
        return find_enrolled_program(connection, &enrollment);
    }

    let program_row = ProgramRow {
        program,
        coach,
        enrollment_id: String::from(""),
        enrollment_status: EnrollmentStatus::NO,
    };

    Ok(vec![program_row])
}

fn find_enrolled_program(connection: &MysqlConnection, enrollment: &Enrollment) -> ProgramResult {
    
    let _program_id = enrollment.program_id.as_str();

    let result: (Program, Coach) = programs.inner_join(coaches).filter(programs::id.eq(&_program_id)).first(connection)?;
   
    let program_row = ProgramRow {
        program:result.0,
        coach:result.1,
        enrollment_id: enrollment.id.to_owned(),
        enrollment_status: EnrollmentStatus::YES,
    };

    Ok(vec![program_row])
}

fn get_enrolled_programs(connection: &MysqlConnection, criteria: &ProgramCriteria) -> ProgramResult {
    type Row = (Enrollment, ProgramType);

    let data: Vec<Row> = enrollments.inner_join(programs.inner_join(coaches)).filter(member_id.eq(&criteria.user_id)).load(connection)?;

    let mut rows: Vec<ProgramRow> = Vec::new();

    for item in data {
        let enrollment = item.0;
        let pc = item.1;
        rows.push(ProgramRow {
            program: pc.0,
            coach: pc.1,
            enrollment_id: enrollment.id,
            enrollment_status: EnrollmentStatus::YES,
        });
    }

    Ok(rows)
}

fn get_coach_programs(connection: &MysqlConnection, criteria: &ProgramCriteria) -> ProgramResult {
    use crate::schema::coaches::dsl::id;

    let data: Vec<ProgramType> = programs.inner_join(coaches).filter(id.eq(&criteria.user_id)).order_by(name.asc()).load(connection)?;

    Ok(to_program_rows(data))
}

fn get_latest_programs(connection: &MysqlConnection) -> ProgramResult {
    use crate::schema::programs::dsl::updated_at;

    let data: Vec<ProgramType> = programs
        .inner_join(coaches)
        .order_by(updated_at.asc())
        .filter(active.eq(true))
        .filter(is_private.eq(false))
        .filter(is_parent.eq(true))
        .limit(10)
        .load(connection)?;

    Ok(to_program_rows(data))
}

fn to_program_rows(data: Vec<ProgramType>) -> Vec<ProgramRow> {
    let mut rows: Vec<ProgramRow> = Vec::new();

    for pc in data {
        rows.push(ProgramRow {
            program: pc.0,
            coach: pc.1,
            enrollment_id: String::from(""),
            enrollment_status: EnrollmentStatus::UNKNOWN,
        });
    }

    rows
}
