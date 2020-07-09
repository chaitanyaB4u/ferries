use crate::models::programs::Program;
use crate::models::users::User;
use diesel::prelude::*;

#[derive(juniper::GraphQLInputObject)]
pub struct ProgramCriteria {
    user_fuzzy_id: String,
}

pub struct ProgramRow {
    pub program:Program,
    pub user:User,
}

#[juniper::object]
impl ProgramRow {

    pub fn program(&self) -> &Program {
        &self.program
    }

    pub fn coach(&self) -> &User {
        &self.user
    }
}

pub fn get_active_programs (connection: &MysqlConnection, criteria: ProgramCriteria) -> Vec<ProgramRow> {
    use crate::schema::programs::dsl::*;
    use crate::schema::users::dsl::*;

    let data: Vec<(Program, User)> = programs.inner_join(users).load(connection).unwrap();

    let mut rows: Vec<ProgramRow> = Vec::new();

    for item in data {
        rows.push(ProgramRow{program:item.0,user:item.1});
    }

    rows
}