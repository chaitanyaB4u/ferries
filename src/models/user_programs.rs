use diesel::prelude::*;

use crate::models::coaches::Coach;
use crate::models::enrollments::Enrollment;
use crate::models::programs::Program;
use crate::models::users::User;

use crate::schema::coaches::dsl::*;
use crate::schema::enrollments::dsl::*;
use crate::schema::programs::dsl::*;
use crate::schema::users::dsl::*;

#[derive(juniper::GraphQLEnum)]
pub enum Desire {
    EXPLORE,
    ENROLLED,
    YOURS,
    SINGLE,
}

#[derive(juniper::GraphQLInputObject)]
pub struct ProgramCriteria {
    user_fuzzy_id: String,
    program_fuzzy_id: String,
    desire: Desire,
}

#[derive(juniper::GraphQLEnum)]
pub enum EnrollmentStatus {
    UNKNOWN,
    YES,
    NO
}

pub struct ProgramRow {
    pub program: Program,
    pub coach: Coach,
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
}

type ProgramType = (Program, Coach);
pub type ProgramResult = Result<Vec<ProgramRow>, diesel::result::Error>;

pub fn get_programs(connection: &MysqlConnection, criteria: &ProgramCriteria) -> ProgramResult {
    match &criteria.desire {
        Desire::EXPLORE => get_latest_programs(connection),
        Desire::ENROLLED => get_enrolled_programs(connection, criteria),
        Desire::YOURS => get_coach_programs(connection, criteria),
        Desire::SINGLE => find_program_by_fuzzy_id(connection, criteria),
    }
}

fn is_enrolled(connection: &MysqlConnection, criteria: &ProgramCriteria) -> EnrollmentStatus {
    use crate::schema::programs::dsl::fuzzy_id;
    use crate::schema::users::dsl::fuzzy_id as user_fuzzy_id;

    let enrollment_data: QueryResult<String> = enrollments
        .inner_join(users)
        .inner_join(programs)
        .filter(user_fuzzy_id.eq(&criteria.user_fuzzy_id))
        .filter(fuzzy_id.eq(&criteria.program_fuzzy_id))
        .select(user_fuzzy_id)
        .first(connection);

    match enrollment_data {
        Ok(_) => EnrollmentStatus::YES,
        Err(_) => EnrollmentStatus::NO,
    }

}

fn find_program_by_fuzzy_id(connection: &MysqlConnection, criteria: &ProgramCriteria) -> ProgramResult {
    use crate::schema::programs::dsl::fuzzy_id;

    let result: (Program,Coach) = programs
        .inner_join(coaches)
        .filter(fuzzy_id.eq(&criteria.program_fuzzy_id))
        .first(connection)?;

    let enrollment_status = is_enrolled(connection, criteria);

    let program_row = ProgramRow {program:result.0, coach:result.1, enrollment_status};

    Ok(vec![program_row])
}

fn get_enrolled_programs(
    connection: &MysqlConnection,
    criteria: &ProgramCriteria,
) -> ProgramResult {
    use crate::schema::users::dsl::fuzzy_id;
    type Row = (Enrollment, User, ProgramType);

    let data: Vec<Row> = enrollments
        .inner_join(users)
        .inner_join(programs.inner_join(coaches))
        .filter(fuzzy_id.eq(&criteria.user_fuzzy_id))
        .load(connection)?;

    let mut rows: Vec<ProgramRow> = Vec::new();

    for item in data {
        let pc = item.2;
        rows.push(ProgramRow {
            program: pc.0,
            coach: pc.1,
            enrollment_status: EnrollmentStatus::YES,
        });
    }

    Ok(rows)
}

fn get_coach_programs(connection: &MysqlConnection, criteria: &ProgramCriteria) -> ProgramResult {
    use crate::schema::coaches::dsl::fuzzy_id;

    let data: Vec<ProgramType> = programs
        .inner_join(coaches)
        .filter(fuzzy_id.eq(&criteria.user_fuzzy_id))
        .order_by(name.asc())
        .load(connection)?;

    Ok(to_program_rows(data))
}

fn get_latest_programs(connection: &MysqlConnection) -> ProgramResult {
    use crate::schema::programs::dsl::created_at;

    let data: Vec<ProgramType> = programs
        .inner_join(coaches)
        .order_by(created_at.asc())
        .filter(active.eq(true))
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
            enrollment_status: EnrollmentStatus::UNKNOWN,
        });
    }

    rows
}
