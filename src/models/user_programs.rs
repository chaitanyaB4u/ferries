use diesel::prelude::*;

use crate::models::coaches::Coach;
use crate::models::enrollments::Enrollment;
use crate::models::programs::Program;

use crate::schema::coaches::dsl::*;
use crate::schema::enrollments::dsl::*;
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

fn get_enrollment(connection: &MysqlConnection, criteria: &ProgramCriteria) -> (String, EnrollmentStatus) {
    use crate::schema::enrollments::dsl::id;

    let enrollment_data: QueryResult<String> = enrollments
        .filter(member_id.eq(&criteria.user_id))
        .filter(program_id.eq(&criteria.program_id))
        .select(id)
        .first(connection);

    match enrollment_data {
        Ok(enrollment_id) => (enrollment_id, EnrollmentStatus::YES),
        Err(_) => (String::from(""), EnrollmentStatus::NO),
    }
}

fn find_program(connection: &MysqlConnection, criteria: &ProgramCriteria) -> ProgramResult {
    use crate::schema::programs::dsl::id;

    let result: (Program, Coach) = programs.inner_join(coaches).filter(id.eq(&criteria.program_id)).first(connection)?;

    let enrollment = get_enrollment(connection, criteria);

    let program_row = ProgramRow {
        program: result.0,
        coach: result.1,
        enrollment_id: enrollment.0,
        enrollment_status: enrollment.1,
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
    use crate::schema::programs::dsl::created_at;

    let data: Vec<ProgramType> = programs.inner_join(coaches).order_by(created_at.asc()).filter(active.eq(true)).limit(10).load(connection)?;

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
