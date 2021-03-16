use diesel::prelude::*;

use crate::commons::util;

use crate::services::enrollments;
use crate::services::programs;
use crate::services::sessions::{find_by_conference, insert_session, insert_session_member, remove_conference_session,create_session_mail};
use crate::services::users;

use crate::models::conferences::{Conference, IntentionState, MemberRequest, NewConference, NewConferenceRequest};
use crate::models::programs::Program;
use crate::models::sessions::{ChangeSessionStateRequest, NewSession, Session, TargetState};
use crate::models::users::User;
use crate::schema::conferences::dsl::*;

const CONFERENCE_CREATION_ERROR: &str = "Unable to create conference.";
const FINDER_ERROR: &str = "Unable to find the conference.";
const CONFERENCE_STATE_UPDATE_ERROR: &str = "Unable to complete the requested action on the state of the conference";

pub fn create_conference(connection: &MysqlConnection, request: &NewConferenceRequest) -> Result<Conference, &'static str> {
    let program = programs::find(connection, request.program_id.as_str())?;

    let coach = users::find(connection, &program.coach_id.as_str())?;

    let people_involved = coach.full_name.to_owned();

    let new_conference = NewConference::from(request, people_involved);

    let conference = insert_conference(connection, &new_conference)?;

    create_coach_session(connection, &conference, &program, &coach)?;

    Ok(conference)
}

pub fn manage_members(connection: &MysqlConnection, member_request: &MemberRequest) -> Result<Vec<String>, &'static str> {
    if let IntentionState::ADD = member_request.intention {
        return add_members(connection, member_request);
    }

    remove_members(connection, member_request)
}

fn insert_conference(connection: &MysqlConnection, new_conference: &NewConference) -> Result<Conference, &'static str> {
    let result = diesel::insert_into(conferences).values(new_conference).execute(connection);

    if result.is_err() {
        return Err(CONFERENCE_CREATION_ERROR);
    }

    find(connection, new_conference.id.as_str())
}

fn find(connection: &MysqlConnection, conf_id: &str) -> Result<Conference, &'static str> {
    let result = conferences.filter(crate::schema::conferences::id.eq(conf_id)).first(connection);

    if result.is_err() {
        return Err(FINDER_ERROR);
    }

    Ok(result.unwrap())
}

fn add_members(connection: &MysqlConnection, member_request: &MemberRequest) -> Result<Vec<String>, &'static str> {
    let conf_id = member_request.conference_id.as_str();

    let conference = find(connection, conf_id)?;

    let program = programs::find(connection, conference.program_id.as_str())?;
    let coach = users::find(connection, program.coach_id.as_str())?;

    let mut added_members: Vec<String> = Vec::new();
    for member_id in &member_request.member_ids {
        let result = find_or_create_session(connection, &conference, member_id, &program, &coach);
        if result.is_ok() {
            added_members.push(member_id.to_owned());
        }
    }

    Ok(added_members)
}

fn find_or_create_session(connection: &MysqlConnection, conference: &Conference, member_id: &str, program: &Program, coach: &User) -> Result<Session, &'static str> {
    if let Ok(session) = find_by_conference(connection, conference.id.as_str(), member_id) {
        return Ok(session);
    }

    let member = users::find(connection, member_id)?;
    let enrollment = enrollments::find(connection, &program, &member)?;

    let is_coach_session = coach.id.as_str().eq(member.id.as_str());

    let mut people_involved = coach.full_name.to_owned();
    let mut user_type = util::COACH;

    if !is_coach_session {
        people_involved = util::concat(coach.full_name.as_str(), member.full_name.as_str());
        user_type = util::MEMBER;
    }

    let fuzzy_id = util::fuzzy_id();

    let new_session = NewSession {
        id: fuzzy_id,
        name: conference.name.to_owned(),
        description: conference.description().to_owned(),
        program_id: conference.program_id.to_owned(),
        enrollment_id: enrollment.id.to_owned(),
        people: people_involved,
        duration: conference.duration,
        original_start_date: conference.original_start_date,
        original_end_date: conference.original_end_date,
        conference_id: Some(conference.id.to_owned()),
        session_type: util::MULTI.to_owned(),
        is_ready: conference.is_ready,
    };

    let session = insert_session(connection, &new_session)?;
    insert_session_member(connection, &session, &member, user_type)?;
    
    enrollments::mark_as_old(connection, enrollment.id())?;

    if !is_coach_session {
        create_session_mail(connection, &session, &member, &coach)?;
    }

    Ok(session)
}

fn remove_members(connection: &MysqlConnection, member_request: &MemberRequest) -> Result<Vec<String>, &'static str> {
    let conf_id = member_request.conference_id.as_str();

    find(connection, conf_id)?;

    let mut _members: Vec<String> = Vec::new();
    for member_id in &member_request.member_ids {
        let result = remove_conference_session(connection, conf_id, &member_id);
        if result.is_ok() {
            _members.push(member_id.to_owned());
        }
    }

    Ok(_members)
}

// Coach Session is a special entry with a self enrollment id
fn create_coach_session(connection: &MysqlConnection, conference: &Conference, program: &Program, coach: &User) -> Result<Session, &'static str> {
    enrollments::find_or_create_coach_enrollment(connection, conference.program_id.as_str())?;

    find_or_create_session(connection, &conference, &coach.id.to_owned(), &program, &coach)
}

// To keep the state of the conference in sync with the coach's session state.
// When a new member is added during a live session, 
// the newly added user sees its state as that of his/her peers.

pub fn sync_conference_state(connection: &MysqlConnection, request: &ChangeSessionStateRequest, conf_id: &str) -> Result<usize, &'static str> {

    let target_conference = conferences.filter(id.eq(conf_id));

    let now = util::now();

    let result = match request.target_state {
        TargetState::READY => diesel::update(target_conference).set(is_ready.eq(true)).execute(connection),
        TargetState::START => diesel::update(target_conference).set(actual_start_date.eq(now)).execute(connection),
        TargetState::DONE => diesel::update(target_conference)
            .set((actual_end_date.eq(now), closing_notes.eq(&request.closing_notes)))
            .execute(connection),
        TargetState::CANCEL => diesel::update(target_conference).set((cancelled_at.eq(now), closing_notes.eq(&request.closing_notes))).execute(connection),
    };

    if result.is_err() {
        return Err(CONFERENCE_STATE_UPDATE_ERROR);
    }

    Ok(result.unwrap())
}
