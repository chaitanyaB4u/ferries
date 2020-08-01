use diesel::prelude::*;

use std::collections::HashMap;

use crate::commons::util;
use crate::models::programs::Program;
use crate::models::session_users::SessionUser;
use crate::models::sessions::Session;
use crate::models::users::User;

use crate::schema::programs::dsl::*;
use crate::schema::session_users;
use crate::schema::session_users::dsl::*;
use crate::schema::sessions;
use crate::schema::sessions::dsl::*;
use crate::schema::users::dsl::*;

#[derive(juniper::GraphQLInputObject)]
pub struct EventCriteria {
    user_fuzzy_id: String,
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

pub fn get_events(connection: &MysqlConnection, criteria: EventCriteria) -> Vec<EventRow> {
    use crate::schema::users::dsl::fuzzy_id;

    let user: User = users
        .filter(fuzzy_id.eq(criteria.user_fuzzy_id))
        .first(connection)
        .unwrap();

    let rows: Vec<(Session, Program, SessionUser)> = sessions
        .inner_join(programs)
        .inner_join(session_users)
        .filter(session_users::user_id.eq(user.id))
        .load(connection)
        .unwrap();
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

#[derive(juniper::GraphQLInputObject)]
pub struct SessionCriteria {
    fuzzy_id: String,
}

pub struct SessionPeople {
    pub coach: Option<User>,
    pub member: Option<User>,
}

#[juniper::object]
impl SessionPeople {
    pub fn coach(&self) -> &Option<User> {
        &self.coach
    }

    pub fn member(&self) -> &Option<User> {
        &self.member
    }
}

impl SessionPeople {
    pub fn new(coach: Option<&User>, member: Option<&User>) -> SessionPeople {
        SessionPeople {
            coach: deref(coach),
            member: deref(member),
        }
    }
}

pub fn deref(a_user: Option<&User>) -> Option<User> {
    let some_user = a_user?;
    Some(some_user.clone())
}

pub type PeopleResult = Result<SessionPeople, diesel::result::Error>;

pub fn get_people(connection: &MysqlConnection, criteria: SessionCriteria) -> PeopleResult {
    let result_session_id = sessions
        .filter(sessions::fuzzy_id.eq(criteria.fuzzy_id))
        .select(sessions::id)
        .first::<i32>(connection)?;

    let result: Vec<(SessionUser, User)> = session_users
        .inner_join(users)
        .filter(session_id.eq(result_session_id))
        .load(connection)?;

    let user_map: HashMap<String, User> = result
        .iter()
        .map(|tuple| (tuple.0.user_type.clone(), tuple.1.clone()))
        .collect();

    let coach = user_map.get(util::COACH);
    let member = user_map.get(util::MEMBER);

    Ok(SessionPeople::new(coach, member))
}
