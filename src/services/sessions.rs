use diesel::prelude::*;

use crate::models::sessions::{NewSessionRequest, NewSession,Session};
use crate::schema::sessions::dsl::*;

const ERROR_001: &'static str = "Session Creation Error:001";
const ERROR_002: &'static str = "Post-Session Creation Error:002. Unable to Create Or Find the Session.";

pub fn create_session(connection: &MysqlConnection, request: &NewSessionRequest) -> Result<Session,&'static str>{

    let new_session = NewSession::from(request);

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
