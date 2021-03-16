use diesel::prelude::*;

use std::collections::HashMap;

use crate::commons::util;

use crate::services::correspondences::create_mail;
use crate::services::enrollments;
use crate::services::programs;
use crate::services::users;

use crate::services::conferences::{sync_conference_state};

use crate::models::correspondences::{MailOut, MailRecipient};
use crate::models::enrollments::Enrollment;
use crate::models::session_users::{NewSessionUser, SessionUser};
use crate::models::sessions::{ChangeSessionStateRequest, NewSession, NewSessionRequest, Session, TargetState};
use crate::models::users::User;

use crate::schema::enrollments::dsl::*;
use crate::schema::session_users::dsl::*;
use crate::schema::sessions::dsl::*;
use crate::schema::users::dsl::*;

const SESSION_CREATION_ERROR: &str = "Unable to Create Session. Error:002";
const SESSION_NOT_FOUND: &str = "Unable to Create Or Find the Session. Error:003.";

const SESSION_USER_CREATION_ERROR: &str = "Unable to associate users to the session. Error: 004.";

const SESSION_STATE_CHANGE_PROHIBITED: &str = "The session is either cancelled or completed. Hence change of state to the session is not permitted.";
const SESSION_UPDATE_ERROR: &str = "Unable to complete the requested action on the state";

const NOT_IN_CONFERENCE: &str = "The member is not included in the conference";
const UNREMOVABLE_SESSION: &str = "The session is not in a removable state";

pub fn create_session(connection: &MysqlConnection, request: &NewSessionRequest) -> Result<Session, &'static str> {
    // Obtain the Program
    let program = programs::find(connection, request.program_id.as_str())?;

    // Obtain the People (We need the User corresponds to the Coach)
    let coach: User = users::find(connection, program.coach_id.as_str())?;

    let member: User = users::find(connection, request.member_id.as_str())?;

    let enrollment: Enrollment = enrollments::find(connection, &program, &member)?;

    let people_involved: String = util::concat(coach.full_name.as_str(), member.full_name.as_str());

    // Inserting the Session
    let new_session = NewSession::from(request, enrollment.id.to_owned(), people_involved);
    let session = insert_session(connection, &new_session)?;

    // Inserting a pair of entries into the Session Users (For Coach & Member)
    let new_session_coach = NewSessionUser::from(&session, &coach, util::COACH);
    let new_session_member = NewSessionUser::from(&session, &member, util::MEMBER);
    insert_session_users(connection, &new_session_coach, &new_session_member)?;

    enrollments::mark_as_old(connection, enrollment.id())?;

    create_session_mail(connection, &session, &member, &coach)?;

    Ok(session)
}

pub fn find_by_conference(connection: &MysqlConnection, conf_id: &str, given_member_id: &str) -> Result<Session, &'static str> {
    
    let result: Result<(Session, Enrollment), diesel::result::Error> = sessions
        .inner_join(enrollments)
        .filter(conference_id.eq(conf_id))
        .filter(member_id.eq(given_member_id))
        .first(connection);

    if result.is_err() {
        return Err(NOT_IN_CONFERENCE);
    }

    Ok(result.unwrap().0)
}

pub fn remove_conference_session(connection: &MysqlConnection, conf_id: &str, given_member_id: &str) -> Result<bool, &'static str> {
    let session = find_by_conference(connection, conf_id, given_member_id)?;

    if !session.can_delete() {
        return Err(UNREMOVABLE_SESSION);
    }

    let _session_id = session.id.as_str();

    let result = diesel::delete(session_users.filter(session_id.eq(_session_id))).execute(connection);
    if result.is_err() {
        return Err(UNREMOVABLE_SESSION);
    }

    use crate::schema::sessions::dsl::id;
    let result = diesel::delete(sessions.filter(id.eq(_session_id))).execute(connection);
    if result.is_err() {
        return Err(UNREMOVABLE_SESSION);
    }

    Ok(true)
}

pub fn find_session_user(connection: &MysqlConnection, session_user_id: &str) -> QueryResult<SessionUser> {
    use crate::schema::session_users::dsl::id;

    session_users.filter(id.eq(session_user_id)).first(connection)
}

pub fn change_session_state(connection: &MysqlConnection, request: &ChangeSessionStateRequest) -> Result<Session, &'static str> {
    let session = can_change_session_state(connection, request)?;

    if session.is_conference() {
        let conf_id = session.conference_id.unwrap();
        do_alter_multi_sessions_state(connection,request,conf_id.as_str())?;
        sync_conference_state(connection,request,conf_id.as_str())?;
    }
    else {
        do_alter_mono_session_state(connection, request)?;    
    }
   
    let session = find(connection, &request.id.as_str())?;
    
    if request.target_state == TargetState::CANCEL && !session.is_conference() {
        send_session_cancel_mail(connection, &session)?;
    }

    Ok(session)
}

fn can_change_session_state(connection: &MysqlConnection, request: &ChangeSessionStateRequest) -> Result<Session, &'static str> {
    let the_id = &request.id.as_str();

    let session = find(connection, the_id)?;

    let flag = session.cancelled_at.is_none() && session.actual_end_date.is_none();

    if !flag {
        return Err(SESSION_STATE_CHANGE_PROHIBITED);
    }

    Ok(session)
}

fn do_alter_multi_sessions_state(connection: &MysqlConnection, request: &ChangeSessionStateRequest, conf_id: &str) -> Result<usize, &'static str> {

    let target_sessions = sessions.filter(conference_id.eq(conf_id));

    let now = util::now();

    let result = match request.target_state {
        TargetState::READY => diesel::update(target_sessions).set(is_ready.eq(true)).execute(connection),
        TargetState::START => diesel::update(target_sessions).set(actual_start_date.eq(now)).execute(connection),
        TargetState::DONE => diesel::update(target_sessions)
            .set((actual_end_date.eq(now), closing_notes.eq(&request.closing_notes)))
            .execute(connection),
        TargetState::CANCEL => diesel::update(target_sessions).set((cancelled_at.eq(now), closing_notes.eq(&request.closing_notes))).execute(connection),
    };

    if result.is_err() {
        return Err(SESSION_UPDATE_ERROR);
    }

    Ok(result.unwrap())
}

fn do_alter_mono_session_state(connection: &MysqlConnection, request: &ChangeSessionStateRequest) -> Result<usize, &'static str> {

    use crate::schema::sessions::dsl::id;
    let the_session_id = &request.id.as_str();
    let target_session = sessions.filter(id.eq(the_session_id));

    let now = util::now();

    let result = match request.target_state {
        TargetState::READY => diesel::update(target_session).set(is_ready.eq(true)).execute(connection),
        TargetState::START => diesel::update(target_session).set(actual_start_date.eq(now)).execute(connection),
        TargetState::DONE => diesel::update(target_session)
            .set((actual_end_date.eq(now), closing_notes.eq(&request.closing_notes)))
            .execute(connection),
        TargetState::CANCEL => diesel::update(target_session).set((cancelled_at.eq(now), closing_notes.eq(&request.closing_notes))).execute(connection),
    };

    if result.is_err() {
        return Err(SESSION_UPDATE_ERROR);
    }

    Ok(result.unwrap())
}

pub fn insert_session(connection: &MysqlConnection, new_session: &NewSession) -> Result<Session, &'static str> {
    let result = diesel::insert_into(sessions).values(new_session).execute(connection);

    if result.is_err() {
        return Err(SESSION_CREATION_ERROR);
    }

    find(connection, new_session.id.as_str())
}

pub fn find(connection: &MysqlConnection, the_id: &str) -> Result<Session, &'static str> {
    use crate::schema::sessions::dsl::id;

    let session_result = sessions.filter(id.eq(the_id)).first(connection);

    if session_result.is_err() {
        return Err(SESSION_NOT_FOUND);
    }

    Ok(session_result.unwrap())
}

pub fn insert_session_users(connection: &MysqlConnection, coach: &NewSessionUser, member: &NewSessionUser) -> Result<usize, &'static str> {
    let result = diesel::insert_into(session_users).values(vec![coach, member]).execute(connection);

    if result.is_err() {
        return Err(SESSION_USER_CREATION_ERROR);
    }

    Ok(result.unwrap())
}

pub fn insert_session_member(connection: &MysqlConnection, session: &Session, member: &User, session_user_type: &str) -> Result<usize, &'static str> {
    let new_session_member = NewSessionUser::from(&session, &member, session_user_type);
    let result = diesel::insert_into(session_users).values(&new_session_member).execute(connection);
    if result.is_err() {
        return Err(SESSION_USER_CREATION_ERROR);
    }
    Ok(result.unwrap())
}


pub fn create_session_mail(connection: &MysqlConnection, session: &Session, member: &User, coach: &User) -> Result<usize, &'static str> {
    let mail_out = MailOut::for_new_session(session, coach, member);
    let recipients = MailRecipient::build_recipients(member, coach, mail_out.id.as_str());

    create_mail(connection, mail_out, recipients)
}

fn send_session_cancel_mail(connection: &MysqlConnection, session: &Session) -> Result<usize, &'static str> {
    let sus: Vec<(SessionUser, User)> = session_users.inner_join(users).filter(session_id.eq(&session.id)).load(connection).unwrap();

    let team: HashMap<String, User> = sus.iter().map(|tuple| (tuple.0.user_type.clone(), tuple.1.clone())).collect();

    let coach = team.get("coach").unwrap();
    let member = team.get("member").unwrap();

    let mail_out = MailOut::for_cancel_session(session, coach, member);
    let recipients = MailRecipient::build_recipients(member, coach, mail_out.id.as_str());
    create_mail(connection, mail_out, recipients)
}
