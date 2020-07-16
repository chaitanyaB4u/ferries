use diesel::prelude::*;

use crate::commons::util;

use crate::services::enrollments;
use crate::services::programs;

use crate::models::session_users::NewSessionUser;
use crate::models::sessions::{NewSession, NewSessionRequest, Session};
use crate::models::users::User;

use crate::schema::session_users::dsl::*;
use crate::schema::sessions::dsl::*;
use crate::schema::users::dsl::*;

const ERROR_001: &'static str = "Session Creation Error:001";
const ERROR_002: &'static str =
    "Post-Session Creation Error:002. Unable to Create Or Find the Session.";
const ERROR_003: &'static str = "Invalid Program Fuzzy Id Error:003.";

pub fn create_session(
    connection: &MysqlConnection,
    request: &NewSessionRequest,
) -> Result<Session, &'static str> {
    
    // Obtain the Program
    let program_result = programs::find_by_fuzzy_id(connection, request.program_fuzzy_id.as_str());

    if program_result.is_err() {
        return Err(ERROR_003);
    }
    let program = program_result.unwrap();

    // Obtain the People
    let member: User = enrollments::get_member(connection, program.id);
    let coach: User = users.find(program.coach_id).first(connection).unwrap();

    let people_involved: String = util::concat(coach.full_name.as_str(), member.full_name.as_str());

    // Inserting the Session
    let new_session = NewSession::from(request, program.id, people_involved);

    let result = diesel::insert_into(sessions)
        .values(&new_session)
        .execute(connection);

    if result.is_err() {
        return Err(ERROR_001);
    }

    let session_result = find_by_fuzzy_id(connection, new_session.fuzzy_id.as_str());

    if session_result.is_err() {
        return Err(ERROR_002);
    }

    let session = session_result.unwrap();
    let new_session_coach = NewSessionUser::into(&session, &coach, "coach");
    let new_session_member = NewSessionUser::into(&session, &member, "member");

    // Inserting two entries into the Session Users
    let session_user_result = diesel::insert_into(session_users)
        .values(vec![&new_session_coach, &new_session_member])
        .execute(connection);

    if session_user_result.is_err() {
        return Err(ERROR_002);
    }

    Ok(session)
}

fn find_by_fuzzy_id(connection: &MysqlConnection, fuzzy: &str) -> QueryResult<Session> {
    use crate::schema::sessions::dsl::fuzzy_id;

    sessions.filter(fuzzy_id.eq(fuzzy)).first(connection)
}
