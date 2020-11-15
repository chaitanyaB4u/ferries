use diesel::prelude::*;

use crate::commons::util;
use chrono::{Duration, NaiveDateTime};

use crate::models::enrollments::PlanCriteria;
use crate::models::tasks::{NewTask, NewTaskRequest, Task, UpdateTask, UpdateClosingNoteRequest, UpdateTaskRequest,UpdateResponseRequest, ChangeMemberTaskStateRequest, ChangeCoachTaskStateRequest, MemberTargetState, CoachTargetState};
use crate::schema::tasks::dsl::*;

const STATE_CHANGE_PROHIBITED: &'static str = "The task is either cancelled or responded.";
const TASK_NOT_FOUND: &'static str = "Unable to find the Task.";
const UPDATE_ERROR: &'static str = "Unable to complete the requested action.";
const UPDATE_NOTES_ERROR: &'static str = "Unable to update the notes.";

pub fn create_task(connection: &MysqlConnection, request: &NewTaskRequest) -> Result<Task, diesel::result::Error> {
    let new_task = NewTask::from(request);

    diesel::insert_into(tasks).values(&new_task).execute(connection)?;

    tasks.filter(id.eq(new_task.id.to_owned())).first(connection)
}

pub fn update_task(connection: &MysqlConnection, request: &UpdateTaskRequest) -> Result<Task, diesel::result::Error> {
    let the_id = &request.id.as_str();

    let start_date = util::as_date(request.start_time.as_str());
    let given_duration = Duration::hours(request.duration as i64);
    let end_date = start_date.checked_add_signed(given_duration);

    diesel::update(tasks.filter(id.eq(the_id)))
        .set(&UpdateTask {
            description: request.description.to_owned(),
            name: request.name.to_owned(),
            duration: request.duration,
            original_start_date: start_date,
            original_end_date: end_date.unwrap_or(start_date),
        })
        .execute(connection)?;

    tasks.filter(id.eq(the_id)).first(connection)
}

pub fn update_closing_notes(connection: &MysqlConnection, request: &UpdateClosingNoteRequest) -> Result<Task, &'static str> {

    let the_id = &request.id.as_str();
    let target = tasks.filter(id.eq(the_id));

    let result = diesel::update(target).set(closing_notes.eq(&request.notes)).execute(connection);

    if result.is_err(){
        return Err(UPDATE_NOTES_ERROR)
    }

    find(connection, the_id)

}

pub fn update_response(connection: &MysqlConnection, request: &UpdateResponseRequest) -> Result<Task, &'static str> {

    can_allow_response_change(connection, request)?;

    let the_id = &request.id.as_str();
    let target_task = tasks.filter(id.eq(the_id));

    let result = diesel::update(target_task).set(response.eq(&request.response)).execute(connection);

    if result.is_err(){
        return Err(UPDATE_ERROR);
    }

    find(connection, the_id)
}

fn can_allow_response_change(connection: &MysqlConnection, request: &UpdateResponseRequest) -> Result <usize, &'static str> {
    let the_id = &request.id.as_str();

    let task = find(connection, the_id)?;

    let flag = task.can_respond();

    if !flag {
        return Err(STATE_CHANGE_PROHIBITED);
    }

    Ok(1)

}

pub fn change_coach_task_state(connection: &MysqlConnection, request: &ChangeCoachTaskStateRequest) -> Result<Task, &'static str> {

    can_allow_coach_task_state_change(connection, request)?;

    let the_id = &request.id.as_str();
    let target_task = tasks.filter(id.eq(the_id));
    let now = util::now();

    let none_date: Option<NaiveDateTime> = None;

    let result = match request.target_state {

        CoachTargetState::CANCEL => diesel::update(target_task).set(cancelled_at.eq(now)).execute(connection),
        CoachTargetState::DONE => diesel::update(target_task).set(actual_end_date.eq(now)).execute(connection),
        CoachTargetState::REOPEN => diesel::update(target_task).set(responded_date.eq(none_date)).execute(connection)
    };

    if result.is_err() {
        return Err(UPDATE_ERROR);
    }

    find(connection, the_id)

}

pub fn change_member_task_state(connection: &MysqlConnection, request: &ChangeMemberTaskStateRequest) -> Result<Task, &'static str> {
    
    can_allow_member_task_state_change(connection, request)?;

    let the_id = &request.id.as_str();
    let target_task = tasks.filter(id.eq(the_id));
    let now = util::now();

    let result = match request.target_state {

        MemberTargetState:: START => diesel::update(target_task).set(actual_start_date.eq(now)).execute(connection),
        MemberTargetState:: FINISH => diesel::update(target_task).set(responded_date.eq(now)).execute(connection)
    };

    if result.is_err() {
        return Err(UPDATE_ERROR);
    }

    find(connection, the_id)
}

fn can_allow_coach_task_state_change(connection: &MysqlConnection, request: &ChangeCoachTaskStateRequest) -> Result<usize, &'static str> {
    let the_id = &request.id.as_str();

    let task = find(connection, the_id)?;

    let result: bool = match request.target_state {
        CoachTargetState::DONE => task.can_complete(),
        CoachTargetState::CANCEL => task.can_cancel(),
        CoachTargetState::REOPEN => task.can_reopen()
    };

    if !result {
        return Err(STATE_CHANGE_PROHIBITED);
    }

    Ok(1)
}

fn can_allow_member_task_state_change(connection: &MysqlConnection, request: &ChangeMemberTaskStateRequest) -> Result<usize, &'static str> {
    let the_id = &request.id.as_str();

    let task = find(connection, the_id)?;

    let result: bool = match request.target_state {
        MemberTargetState::START => task.can_start(),
        MemberTargetState::FINISH => task.can_finish()
    };

    if !result {
        return Err(STATE_CHANGE_PROHIBITED);
    }

    Ok(1)
}

fn find(connection: &MysqlConnection, the_id: &str) -> Result<Task, &'static str> {
    let result = tasks.filter(id.eq(the_id)).first(connection);

    if result.is_err() {
        return Err(TASK_NOT_FOUND);
    }

    Ok(result.unwrap())

}

pub fn get_tasks(connection: &MysqlConnection, criteria: PlanCriteria) -> Result<Vec<Task>, diesel::result::Error> {
    tasks
        .filter(enrollment_id.eq(criteria.enrollment_id))
        .order_by(original_start_date.asc())
        .load(connection)
}