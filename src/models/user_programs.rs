use diesel::prelude::*;

use crate::models::enrollments::Enrollment;
use crate::models::programs::Program;
use crate::models::users::User;

use crate::schema::enrollments::dsl::*;
use crate::schema::programs::dsl::*;
use crate::schema::users::dsl::*;


#[derive(juniper::GraphQLEnum)]
pub enum Desire {
    EXPLORE,
    ENROLLED,
    YOURS,
}

#[derive(juniper::GraphQLInputObject)]
pub struct ProgramCriteria {
    user_fuzzy_id: String,
    desire: Desire,
}

pub fn get_programs(connection: &MysqlConnection, criteria: &ProgramCriteria) -> Vec<Program> {
    match &criteria.desire {
        Desire::EXPLORE => get_latest_programs(connection),
        Desire::ENROLLED => get_enrolled_programs(connection, criteria),
        Desire::YOURS => get_coach_programs(connection, criteria),
    }
}

pub fn get_enrolled_programs(connection: &MysqlConnection,criteria: &ProgramCriteria) -> Vec<Program> {
    use crate::schema::users::dsl::fuzzy_id;
    type Row = (Enrollment, User, Program);

    let data: Vec<Row> = enrollments
        .inner_join(users)
        .inner_join(programs)
        .filter(fuzzy_id.eq(&criteria.user_fuzzy_id))
        .load(connection)
        .unwrap();

    let mut rows: Vec<Program> = Vec::new();
    for item in data {
        rows.push(item.2);
    }
    rows
}

pub fn get_coach_programs(connection: &MysqlConnection,criteria: &ProgramCriteria) -> Vec<Program> {
    use crate::schema::users::dsl::fuzzy_id;

    let data: Vec<(Program, User)> = programs
        .inner_join(users)
        .filter(fuzzy_id.eq(&criteria.user_fuzzy_id))
        .order_by(name.asc())
        .load(connection)
        .unwrap();

    let mut rows: Vec<Program> = Vec::new();

    for item in data {
        rows.push(item.0);
    }

    rows
}

fn get_latest_programs(connection: &MysqlConnection)->Vec<Program> {

    use crate::schema::programs::dsl::created_at;

    programs
    .order_by(created_at.asc())
    .filter(active.eq(true))
    .limit(10)
    .load(connection)
    .unwrap()
}

