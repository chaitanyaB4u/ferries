use diesel::prelude::*;

use crate::commons::util;

use crate::models::enrollments::Enrollment;
use crate::models::notes::Note;
use crate::models::objectives::Objective;
use crate::models::programs::Program;
use crate::models::session_users::SessionUser;
use crate::models::sessions::Session;
use crate::models::tasks::Task;
use crate::models::users::User;

use crate::schema::enrollments;
use crate::schema::enrollments::dsl::*;
use crate::schema::programs::dsl::*;

use crate::schema::objectives;
use crate::schema::objectives::dsl::*;
use crate::schema::session_notes;
use crate::schema::session_notes::dsl::*;
use crate::schema::session_users;
use crate::schema::session_users::dsl::*;
use crate::schema::sessions;
use crate::schema::sessions::dsl::*;
use crate::schema::tasks;
use crate::schema::tasks::dsl::*;
use crate::schema::users::dsl::*;

pub const BAD_QUERY: &'static str = "Error in executing the query";

#[derive(juniper::GraphQLInputObject)]
pub struct EventCriteria {
    pub user_id: String,
    pub program_id: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

pub struct EventRow {
    pub session: Session,
    pub program: Program,
    pub session_user: SessionUser,
}

#[juniper::object]
impl EventRow {
    pub fn session(&self) -> &Session {
        &self.session
    }

    pub fn program(&self) -> &Program {
        &self.program
    }

    pub fn sessionUser(&self) -> &SessionUser {
        &self.session_user
    }
}

type SessionProgram = (Session, Program, SessionUser);

pub fn get_events(connection: &MysqlConnection, criteria: EventCriteria) -> Result<Vec<EventRow>, String> {
    let mut query = sessions
        .inner_join(programs)
        .inner_join(session_users)
        .filter(session_users::user_id.eq(criteria.user_id))
        .order_by(sessions::original_start_date.asc())
        .into_boxed();

    if criteria.start_date.is_some() {
        let start_date = util::as_start_date(criteria.start_date.unwrap().as_str())?;
        query = query.filter(sessions::original_start_date.ge(start_date))
    }
    if criteria.end_date.is_some() {
        let end_date = util::as_end_date(criteria.end_date.unwrap().as_str())?;
        query = query.filter(sessions::original_start_date.le(end_date))
    }

    if criteria.program_id.is_some() {
        let prog_id = criteria.program_id.unwrap();
        query = query.filter(sessions::program_id.eq(prog_id));
    }

    let result: Result<Vec<SessionProgram>, diesel::result::Error> = query.load(connection);

    if result.is_err() {
        return Err(BAD_QUERY.to_owned());
    }

    let rows: Vec<SessionProgram> = result.unwrap();
    Ok(to_event_rows(rows))
}

fn to_event_rows(rows: Vec<SessionProgram>) -> Vec<EventRow> {
    let mut event_rows: Vec<EventRow> = Vec::new();

    for row in rows {
        event_rows.push(EventRow {
            session: row.0,
            program: row.1,
            session_user: row.2,
        });
    }

    event_rows
}

pub struct PlanRow {
    pub objective: Option<Objective>,
    pub task: Option<Task>,
    pub note: Option<Note>,
    pub program: Program,
}

#[juniper::object]
impl PlanRow {
    pub fn objective(&self) -> &Option<Objective> {
        &self.objective
    }
    pub fn task(&self) -> &Option<Task> {
        &self.task
    }

    pub fn note(&self) -> &Option<Note> {
        &self.note
    }

    pub fn program(&self) -> &Program {
        &self.program
    }
}

type TaskRowType = (Task, (Enrollment, Program));
fn get_task_events(connection: &MysqlConnection, criteria: &EventCriteria) -> Result<Vec<TaskRowType>, String> {
    let mut query = tasks
        .inner_join(enrollments.inner_join(programs))
        .filter(member_id.eq(&criteria.user_id))
        .order_by(tasks::original_start_date.asc())
        .into_boxed();

    if criteria.start_date.is_some() {
        let start_date = criteria.start_date.as_ref().unwrap().as_str();
        let date = util::as_start_date(start_date)?;
        query = query.filter(tasks::original_start_date.ge(date));
    }

    if criteria.end_date.is_some() {
        let end_date = criteria.end_date.as_ref().unwrap().as_str();
        let date = util::as_end_date(end_date)?;
        query = query.filter(tasks::original_start_date.le(date));
    }

    if criteria.program_id.is_some() {
        let prog_id = criteria.program_id.as_ref().unwrap().as_str();
        query = query.filter(enrollments::program_id.eq(prog_id));
    }

    let result: Result<Vec<TaskRowType>, diesel::result::Error> = query.load(connection);

    if result.is_err() {
        return Err(BAD_QUERY.to_owned());
    }

    Ok(result.unwrap())
}


type ObjectiveRowType = (Objective, (Enrollment, Program));
fn get_objective_events(connection: &MysqlConnection, criteria: &EventCriteria) -> Result<Vec<ObjectiveRowType>, String> {
    let mut query = objectives
        .inner_join(enrollments.inner_join(programs))
        .filter(member_id.eq(&criteria.user_id))
        .order_by(objectives::original_start_date.asc())
        .into_boxed();

    if criteria.start_date.is_some() {
        let start_date = criteria.start_date.as_ref().unwrap().as_str();
        let date = util::as_start_date(start_date)?;
        query = query.filter(objectives::original_start_date.ge(date));
    }
    if criteria.end_date.is_some() {
        let end_date = criteria.end_date.as_ref().unwrap().as_str();
        let date = util::as_end_date(end_date)?;
        query = query.filter(objectives::original_start_date.le(date));
    }

    let result: Result<Vec<ObjectiveRowType>, diesel::result::Error> = query.load(connection);

    if result.is_err() {
        return Err(BAD_QUERY.to_owned());
    }

    Ok(result.unwrap())
}

type NoteRowType = (Note, (Session, Program));
fn get_notes_events(connection: &MysqlConnection, criteria: &EventCriteria) -> Result<Vec<NoteRowType>, String> {
    let mut query = session_notes
        .inner_join(sessions.inner_join(programs))
        .filter(created_by_id.eq(&criteria.user_id))
        .filter(remind_at.is_not_null())
        .into_boxed();

    if criteria.start_date.is_some() {
        let start_date = criteria.start_date.as_ref().unwrap().as_str();
        let date = util::as_start_date(start_date)?;
        query = query.filter(session_notes::remind_at.ge(date))
    }
    if criteria.end_date.is_some() {
        let end_date = criteria.end_date.as_ref().unwrap().as_str();
        let date = util::as_end_date(end_date)?;
        query = query.filter(session_notes::remind_at.le(date))
    }

    let result: Result<Vec<NoteRowType>, diesel::result::Error> = query.load(connection);

    if result.is_err() {
        return Err(BAD_QUERY.to_owned());
    }

    Ok(result.unwrap())
}

pub fn get_plan_events(connection: &MysqlConnection, criteria: EventCriteria) -> Result<Vec<PlanRow>, String> {
    let mut plan_rows: Vec<PlanRow> = Vec::new();

    let objective_rows: Vec<ObjectiveRowType> = get_objective_events(connection, &criteria)?;
    let task_rows: Vec<TaskRowType> = get_task_events(connection, &criteria)?;
    let note_rows: Vec<NoteRowType> = get_notes_events(connection, &criteria)?;

    for row in objective_rows {
        plan_rows.push(PlanRow {
            objective: Some(row.0),
            task: None,
            note: None,
            program: (row.1).1,
        });
    }

    for row in task_rows {
        plan_rows.push(PlanRow {
            task: Some(row.0),
            objective: None,
            note: None,
            program: (row.1).1,
        });
    }

    for row in note_rows {
        plan_rows.push(PlanRow {
            note: Some(row.0),
            objective: None,
            task: None,
            program: (row.1).1,
        });
    }

    Ok(plan_rows)
}

/***
 * A task is due for a member, when the member is yet to responded 
 * and the original end date (planned end date) is on or before the given date.
 * 
 * We consider the end date as a reference point
 */
fn get_member_due_tasks(connection: &MysqlConnection, criteria: &EventCriteria) -> Result<Vec<TaskRowType>, String> {
    let mut query = tasks
        .inner_join(enrollments.inner_join(programs))
        .filter(member_id.eq(&criteria.user_id))
        .filter(tasks::responded_date.is_null())
        .order_by(tasks::original_start_date.asc())
        .into_boxed();

    if criteria.end_date.is_some() {
        let end_date = criteria.end_date.as_ref().unwrap().as_str();
        let date = util::as_end_date(end_date)?;
        query = query.filter(tasks::original_end_date.le(date));
    }

    let result: Result<Vec<TaskRowType>, diesel::result::Error> = query.load(connection);

    if result.is_err() {
        return Err(BAD_QUERY.to_owned());
    }

    Ok(result.unwrap())
}

/***
 * Normally, a task become due for a coach the moment a member responded to the task.
 * 
 * Luckily if a member closes a task before the planned end_date, 
 * the coach gains some time to review.
 * 
 */

type CoachTaskRowType = (Task, User, (Enrollment, Program));
fn get_coach_due_tasks(connection: &MysqlConnection, criteria: &EventCriteria) -> Result<Vec<CoachTaskRowType>, String> {
    let mut query = tasks
        .inner_join(users)
        .inner_join(enrollments.inner_join(programs))
        .filter(coach_id.eq(&criteria.user_id))
        .filter(tasks::responded_date.is_not_null())
        .filter(tasks::actual_end_date.is_null())
        .order_by(tasks::original_start_date.asc())
        .into_boxed();

    if criteria.end_date.is_some() {
        let end_date = criteria.end_date.as_ref().unwrap().as_str();
        let date = util::as_end_date(end_date)?;
        query = query.filter(tasks::original_end_date.le(date));
    }

    let result: Result<Vec<CoachTaskRowType>, diesel::result::Error> = query.load(connection);

    if result.is_err() {
        return Err(BAD_QUERY.to_owned());
    }

    Ok(result.unwrap())
}

pub struct ToDo {
    pub task: Task,
    pub program: Program,
    pub user: Option<User>,
}

#[juniper::object]
impl ToDo {

    pub fn task(&self) -> &Task {
        &self.task
    }
   
    pub fn program(&self) -> &Program {
        &self.program
    }

    pub fn user(&self) -> &Option<User> {
        &self.user
    }
}

pub fn get_to_dos(connection: &MysqlConnection, criteria: EventCriteria) -> Result<Vec<ToDo>, String> {

    let member_tasks = get_member_due_tasks(connection,&criteria)?;
    let coach_tasks = get_coach_due_tasks(connection, &criteria)?;

    let mut to_dos: Vec<ToDo> = Vec::new();

    for row in member_tasks {
        let to_do = ToDo{program:(row.1).1, task:row.0, user:None};
        to_dos.push(to_do);
    }

    for row in coach_tasks {
        let to_do = ToDo{program:(row.2).1, task:row.0, user:Some(row.1)};
        to_dos.push(to_do);
    }

    Ok(to_dos)
}


#[derive(juniper::GraphQLInputObject)]
pub struct SessionCriteria {
    pub id: String,
}

pub struct SessionPeople {
    pub session_user: SessionUser,
    pub user: User,
}

#[juniper::object]
impl SessionPeople {
    pub fn session_user(&self) -> &SessionUser {
        &self.session_user
    }

    pub fn user(&self) -> &User {
        &self.user
    }
}

pub type PeopleResult = Result<Vec<SessionPeople>, diesel::result::Error>;

pub fn get_people(connection: &MysqlConnection, criteria: SessionCriteria) -> PeopleResult {
    use crate::schema::session_users::session_id;

    let result: Vec<(SessionUser, User)> = session_users.inner_join(users).filter(session_id.eq(criteria.id)).load(connection)?;

    let session_people: Vec<SessionPeople> = result
        .iter()
        .map(|tuple| SessionPeople {
            session_user: tuple.0.clone(),
            user: tuple.1.clone(),
        })
        .collect();

    Ok(session_people)
}
