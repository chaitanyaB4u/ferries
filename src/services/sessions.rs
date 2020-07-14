use diesel::prelude::*;

use crate::services::programs;

use crate::models::sessions::{NewSessionRequest, NewSession,Session};
use crate::schema::sessions::dsl::*;

const ERROR_001: &'static str = "Session Creation Error:001";
const ERROR_002: &'static str = "Post-Session Creation Error:002. Unable to Create Or Find the Session.";
const ERROR_003: &'static str = "Invalid Program Fuzzy Id Error:003.";

pub fn create_session(connection: &MysqlConnection, request: &NewSessionRequest) -> Result<Session,&'static str>{


    let program_result = programs::find_by_fuzzy_id(connection, request.program_fuzzy_id.as_str());

    if program_result.is_err() {
        return Err(ERROR_003);
    }

    let new_session = NewSession::from(request, program_result.unwrap().id);

    let result  = diesel::insert_into(sessions)
        .values(&new_session)
        .execute(connection);

    if result.is_err() {
        return Err(ERROR_001);
    }    

    let finder_result = find_by_fuzzy_id(connection, new_session.fuzzy_id.as_str());

    match finder_result {
            Ok(session) => Ok(session),
            Err(_) => {
                return Err(ERROR_002);
            }
    } 
}

fn find_by_fuzzy_id(connection: &MysqlConnection,fuzzy: &str) -> QueryResult<Session> {
    sessions
        .filter(fuzzy_id.eq(fuzzy))
        .first(connection)
}
