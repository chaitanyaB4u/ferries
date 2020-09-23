use diesel::prelude::*;

use crate::models::enrollments::Enrollment;
use crate::models::objectives::Objective;
use crate::models::programs::Program;
use crate::models::session_users::SessionUser;
use crate::models::sessions::Session;
use crate::models::tasks::Task;
use crate::models::users::User;
use crate::models::notes::Note;

use crate::schema::enrollments::dsl::*;
use crate::schema::objectives::dsl::*;
use crate::schema::programs::dsl::*;
use crate::schema::session_users;
use crate::schema::session_users::dsl::*;
use crate::schema::sessions;
use crate::schema::sessions::dsl::*;
use crate::schema::tasks::dsl::*;
use crate::schema::users::dsl::*;
use crate::schema::session_notes::dsl::*;

#[derive(juniper::GraphQLInputObject)]
pub struct EventCriteria {
    user_id: String,
    program_id: Option<String>,
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

pub fn get_events(connection: &MysqlConnection,criteria: EventCriteria) -> Result<Vec<EventRow>, diesel::result::Error> {

    let mut query = sessions
        .inner_join(programs)
        .inner_join(session_users)
        .filter(session_users::user_id.eq(criteria.user_id))
        .order_by(sessions::original_start_date.asc())
        .into_boxed();

    if criteria.program_id.is_some() {
        let prog_id = criteria.program_id.unwrap();
        query = query.filter(sessions::program_id.eq(prog_id));
    }
    
    let rows: Vec<SessionProgram> = query.load(connection)?;

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

    pub fn note(&self) -> &Option<Note>{
        &self.note
    }

    pub fn program(&self) -> &Program {
        &self.program
    }
}

type ObjectiveRowType = (Objective, (Enrollment, Program));
type TaskRowType = (Task, (Enrollment, Program)); 
type NoteRowType = (Note, (Session,Program));

pub fn get_plan_events(connection: &MysqlConnection,criteria: EventCriteria) -> Result<Vec<PlanRow>, diesel::result::Error> {

    let mut plan_rows: Vec<PlanRow> = Vec::new();

    let objective_rows: Vec<ObjectiveRowType> = objectives
        .inner_join(enrollments.inner_join(programs))
        .filter(member_id.eq(&criteria.user_id))
        .load(connection)?;

    let task_rows: Vec<TaskRowType> = tasks
        .inner_join(enrollments.inner_join(programs))
        .filter(member_id.eq(&criteria.user_id))
        .load(connection)?;

    let note_rows: Vec<NoteRowType> = session_notes
        .inner_join(sessions.inner_join(programs))
        .filter(created_by_id.eq(&criteria.user_id))
        .filter(remind_at.is_not_null())
        .load(connection)?;

    for row in objective_rows {
        plan_rows.push(PlanRow {
            objective: Some(row.0),
            task: None,
            note: None,
            program: (row.1).1
        });
    }

    for row in task_rows {
        plan_rows.push(PlanRow {
            task: Some(row.0), 
            objective: None, 
            note: None, 
            program: (row.1).1
        });
    }

    for row in note_rows {
        plan_rows.push(PlanRow {
            note: Some(row.0),
            objective: None,
            task: None,
            program: (row.1).1
        });
    }


   Ok(plan_rows)
}

#[derive(juniper::GraphQLInputObject)]
pub struct SessionCriteria {
    id: String,
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

    let result: Vec<(SessionUser, User)> = session_users
        .inner_join(users)
        .filter(session_id.eq(criteria.id))
        .load(connection)?;

    let session_people: Vec<SessionPeople> = result
        .iter()
        .map(|tuple| SessionPeople {
            session_user: tuple.0.clone(),
            user: tuple.1.clone(),
        })
        .collect();

    Ok(session_people)
}
