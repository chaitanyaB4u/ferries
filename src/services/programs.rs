use diesel::prelude::*;


use crate::services::users;

use crate::models::users::User;
use crate::models::programs::{NewProgramRequest, NewProgram,Program};
use crate::schema::programs::dsl::*;

const ERROR_001: &'static str = "Program Creation. Error:001";
const ERROR_002: &'static str = "Post-Program Creation. Error:002. Unable to Create Or Find the Program.";
const ERROR_003: &'static str = "Invalid Coach Fuzzy Id. Error:003";

pub fn create_new_program(connection: &MysqlConnection, request: &NewProgramRequest) -> Result<Program,&'static str> {

    let user_result = users::find_by_fuzzy_id(connection, request.coach_fuzzy_id.as_str());
    if user_result.is_err() {
        return Err(ERROR_003);
    }
    let user: User = user_result.unwrap();
    let new_program = NewProgram::from(request,user.id);

    let result = diesel::insert_into(programs)
        .values(&new_program)
        .execute(connection);

    if result.is_err() {
        return Err(ERROR_001);
    }       

    let finder_result = find_by_fuzzy_id(connection, new_program.fuzzy_id.as_str());

    match finder_result {
            Ok(program) => Ok(program),
            Err(_) => {
                return Err(ERROR_002);
            }
    } 
}

fn find_by_fuzzy_id(connection: &MysqlConnection,fuzzy: &str) -> QueryResult<Program> {
    programs
        .filter(fuzzy_id.eq(fuzzy))
        .first(connection)
}