use diesel::prelude::*;

use crate::commons::util;

use crate::services::programs;
use crate::services::users;

use crate::models::session_users::{NewSessionUser,SessionUser};
use crate::models::sessions::{NewSession, NewSessionRequest, Session, ChangeSessionStateRequest,TargetState};
use crate::models::users::{User};
use crate::models::coaches::{Coach};

use crate::schema::session_users::dsl::*;
use crate::schema::sessions::dsl::*;
use crate::schema::users::dsl::*;
use crate::schema::coaches::dsl::*;

const SESSION_CREATION_ERROR: &'static str = "Unable to Create Session. Error:002";
const SESSION_NOT_FOUND: &'static str = "Unable to Create Or Find the Session. Error:003.";

const SESSION_USER_CREATION_ERROR: &'static str = "Unable to associate users to the session. Error: 004.";

const SESSION_STATE_CHANGE_PROHIBITED: &'static str = "The session is either cancelled or completed. Hence change of state to the session is not permitted.";
const SESSION_UPDATE_ERROR: &'static str = "Unable to complete the requested action on the state";

pub fn create_session(connection: &MysqlConnection, request: &NewSessionRequest,) -> Result<Session, &'static str> {

    // Obtain the Program
    let program = programs::find_by_fuzzy_id(connection, request.program_fuzzy_id.as_str())?;

    // Obtain the People (We need the User corresponds to the Coach)
    let coach: Coach = coaches.find(program.coach_id).first(connection).unwrap();
    let coach: User = users.find(coach.user_id).first(connection).unwrap();
    
    let member: User = users::find_by_fuzzy_id(connection, request.member_fuzzy_id.as_str())?;

    let people_involved: String = util::concat(coach.full_name.as_str(), member.full_name.as_str());

    // Inserting the Session
    let new_session = NewSession::from(request, program.id, people_involved);
    let session = insert_session(connection,&new_session)?;

    // Inserting a pair of entries into the Session Users (For Coach & Member)
    let new_session_coach = NewSessionUser::from(&session, &coach, util::COACH);
    let new_session_member = NewSessionUser::from(&session, &member,util::MEMBER);
    insert_session_users(connection, &new_session_coach, &new_session_member)?;

    Ok(session)
}

pub fn find_session_user(connection: &MysqlConnection, session_user_fuzzy_id: &str) -> QueryResult<SessionUser> {
    use crate::schema::session_users::dsl::fuzzy_id;

    session_users.filter(fuzzy_id.eq(session_user_fuzzy_id)).first(connection)
}

pub fn change_session_state(connection: &MysqlConnection, request: &ChangeSessionStateRequest) -> Result<Session, &'static str> {
    can_change_session_state(connection,request)?;
    
    do_alter_session_state(connection,request)?;

    find_by_fuzzy_id(connection,&request.fuzzy_id.as_str())
}

fn can_change_session_state(connection: &MysqlConnection, request: &ChangeSessionStateRequest) -> Result<usize, &'static str> {
    let session_fuzzy_id = &request.fuzzy_id.as_str();

    let session = find_by_fuzzy_id(connection,session_fuzzy_id)?;

    let flag = session.cancelled_at.is_none() && session.actual_end_date.is_none();

    if !flag {
        return Err(SESSION_STATE_CHANGE_PROHIBITED);
    }

    Ok(1)
}

fn do_alter_session_state(connection: &MysqlConnection, request: &ChangeSessionStateRequest) -> Result<usize, &'static str> {
    
    use crate::schema::sessions::dsl::fuzzy_id;

    let session_fuzzy_id = &request.fuzzy_id.as_str();
    let target_session = sessions.filter(fuzzy_id.eq(session_fuzzy_id));
    let now = util::now();

    let result = match request.target_state {
        TargetState::READY => {
            diesel::update(target_session).set(is_ready.eq(true)).execute(connection)
        },
        TargetState::START => {
            diesel::update(target_session).set(actual_start_date.eq(now)).execute(connection)
        }
        TargetState::DONE => {
            diesel::update(target_session).set(actual_end_date.eq(now)).execute(connection)
        }
        TargetState::CANCEL => {
            diesel::update(target_session).set(cancelled_at.eq(now)).execute(connection)
        }
    };

    if result.is_err() {
        return Err(SESSION_UPDATE_ERROR);
    }

    Ok(result.unwrap())
}


fn insert_session(connection: &MysqlConnection,new_session: &NewSession) -> Result<Session, &'static str> {

    let result = diesel::insert_into(sessions).values(new_session).execute(connection);

    if result.is_err() {
        return Err(SESSION_CREATION_ERROR);
    }
 
    find_by_fuzzy_id(connection, new_session.fuzzy_id.as_str())
}

fn find_by_fuzzy_id(connection: &MysqlConnection, session_fuzzy_id: &str) -> Result<Session, &'static str> {
    use crate::schema::sessions::dsl::fuzzy_id;

    let session_result = sessions.filter(fuzzy_id.eq(session_fuzzy_id)).first(connection);

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



