use diesel::prelude::*;

use crate::models::programs::Program;
use crate::models::session_users::SessionUser;
use crate::models::sessions::Session;
use crate::models::users::User;
use crate::models::enrollments::{PlanCriteria};

use crate::schema::programs::dsl::*;
use crate::schema::session_users;
use crate::schema::session_users::dsl::*;
use crate::schema::sessions;
use crate::schema::sessions::dsl::*;
use crate::schema::users::dsl::*;

use crate::schema::enrollments;
use crate::schema::enrollments::dsl::*;

#[derive(juniper::GraphQLInputObject)]
pub struct EventCriteria {
    user_id: String,
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

pub fn get_events(connection: &MysqlConnection, criteria: EventCriteria) -> Result<Vec<EventRow>,diesel::result::Error> {

    let rows: Vec<(Session, Program, SessionUser)> = sessions
        .inner_join(programs)
        .inner_join(session_users)
        .filter(session_users::user_id.eq(criteria.user_id))
        .load(connection)?;

    let mut event_rows: Vec<EventRow> = Vec::new();

    for row in rows {
        event_rows.push(EventRow {
            session: row.0,
            program: row.1,
            session_user: row.2,
        });
    }

    Ok(event_rows)
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
    
    let result: Vec<(SessionUser, User)> = session_users
        .inner_join(users)
        .filter(session_id.eq(criteria.id))
        .load(connection)?;

    let session_people: Vec<SessionPeople> = result
        .iter()
        .map(|tuple| SessionPeople{session_user:tuple.0.clone(), user:tuple.1.clone()})
        .collect();

    Ok(session_people)
}

pub fn get_actor_sessions(connection: &MysqlConnection, criteria: PlanCriteria) -> Result<Vec<EventRow>,diesel::result::Error> {
    
    let actor_id: String = enrollments
                    .filter(enrollments::id.eq(&criteria.enrollment_id))
                    .select(member_id)
                    .first(connection)?;
                

    let rows: Vec<(Session, Program, SessionUser)> = sessions
        .inner_join(programs)
        .inner_join(session_users)
        .filter(sessions::enrollment_id.eq(&criteria.enrollment_id))
        .filter(session_users::user_id.eq(actor_id))
        .load(connection)?;
        

    let mut event_rows: Vec<EventRow> = Vec::new();

    for row in rows {
        event_rows.push(EventRow {
            session: row.0,
            program: row.1,
            session_user: row.2,
        });
    }

    Ok(event_rows)
    
}