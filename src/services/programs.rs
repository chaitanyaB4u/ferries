use diesel::prelude::*;

use crate::models::programs::{NewProgramRequest, NewProgram,Program};
use crate::schema::programs::dsl::*;

const ERROR_001: &'static str = "Program Creation. Error:001";
const ERROR_002: &'static str = "Post-Program Creation. Error:002. Unable to Create Or Find the Program.";


pub fn create_new_program(connection: &MysqlConnection, request: &NewProgramRequest) -> Result<Program,&'static str> {

    let new_program = NewProgram::from(request);

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