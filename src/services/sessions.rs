use diesel::prelude::*;

use crate::commons::util;

use crate::services::enrollments;
use crate::services::programs;

use crate::models::programs::Program;
use crate::models::session_users::NewSessionUser;
use crate::models::sessions::{NewSession, NewSessionRequest, Session};
use crate::models::users::{User,UserType};

use crate::schema::session_users::dsl::*;
use crate::schema::sessions::dsl::*;
use crate::schema::users::dsl::*;

const INVALID_PROGRAM: &'static str = "Invalid Program Fuzzy Id. Error:001.";
const SESSION_CREATION_ERROR: &'static str = "Unable to Create Session. Error:002";
const SESSION_NOT_FOUND: &'static str = "Unable to Create Or Find the Session. Error:003.";
const SESSION_USER_CREATION_ERROR: &'static str = "Unable to associate users to the session. Error: 004.";


pub fn create_session(connection: &MysqlConnection, request: &NewSessionRequest,) -> Result<Session, &'static str> {

    // Obtain the Program
    let program = get_program(connection, request)?;

    // Obtain the People
    let coach: User = users.find(program.coach_id).first(connection).unwrap();
    let member: User = enrollments::get_member(connection, program.id);
    let people_involved: String = util::concat(coach.full_name.as_str(), member.full_name.as_str());

    // Inserting the Session
    let new_session = NewSession::from(request, program.id, people_involved);
    let session = insert_session(connection,&new_session)?;

    // Inserting a pair of entries into the Session Users (For Coach & Member)
    let new_session_coach = NewSessionUser::into(&session, &coach, UserType::COACH);
    let new_session_member = NewSessionUser::into(&session, &member, UserType::MEMBER);
    insert_session_users(connection, &new_session_coach, &new_session_member)?;

    Ok(session)
}

fn get_program(connection: &MysqlConnection,request: &NewSessionRequest) -> Result<Program, &'static str> {

    let program_result = programs::find_by_fuzzy_id(connection, request.program_fuzzy_id.as_str());
    
    if program_result.is_err() {
        return Err(INVALID_PROGRAM);
    }

    Ok(program_result.unwrap())
}

fn insert_session(connection: &MysqlConnection,new_session: &NewSession) -> Result<Session, &'static str> {

    let result = diesel::insert_into(sessions).values(new_session).execute(connection);

    if result.is_err() {
        return Err(SESSION_CREATION_ERROR);
    }
 
    find_by_fuzzy_id(connection, new_session.fuzzy_id.as_str())
}

fn find_by_fuzzy_id(connection: &MysqlConnection, fuzzy: &str) -> Result<Session, &'static str> {
    use crate::schema::sessions::dsl::fuzzy_id;

    let session_result = sessions.filter(fuzzy_id.eq(fuzzy)).first(connection);

    if session_result.is_err() {
        return Err(SESSION_NOT_FOUND);
    }

    Ok(session_result.unwrap())
}

fn insert_session_users(connection: &MysqlConnection, coach: &NewSessionUser, member: &NewSessionUser) -> Result<usize,&'static str> {

    let result = diesel::insert_into(session_users)
        .values(vec![coach, member])
        .execute(connection);

    if result.is_err() {
        return Err(SESSION_USER_CREATION_ERROR);
    }

    Ok(result.unwrap())
}

