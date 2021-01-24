use diesel::prelude::*;

use crate::commons::util;

use crate::schema::session_users;
use crate::schema::session_users::dsl::*;
use crate::schema::sessions;
use crate::schema::sessions::dsl::*;
use crate::schema::users::dsl::*;

use crate::models::sessions::Session;
use crate::models::users::User;

#[derive(Clone, Queryable, Debug, Identifiable)]
pub struct SessionUser {
    pub id: String,
    pub session_id: String,
    pub user_id: String,
    pub user_type: String,
}

// Fields that we can safely expose to APIs
#[juniper::object]
impl SessionUser {
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn session_id(&self) -> &str {
        self.session_id.as_str()
    }

    pub fn user_id(&self) -> &str {
        self.user_id.as_str()
    }

    pub fn user_type(&self) -> &str {
        self.user_type.as_str()
    }
}

#[derive(Insertable)]
#[table_name = "session_users"]
pub struct NewSessionUser {
    pub id: String,
    pub session_id: String,
    pub user_id: String,
    pub user_type: String,
}

impl NewSessionUser {
    pub fn from(session: &Session, user: &User, session_user_type: &str) -> NewSessionUser {
        let fuzzy_id = util::fuzzy_id();

        NewSessionUser {
            id: fuzzy_id,
            session_id: session.id.to_owned(),
            user_id: user.id.to_owned(),
            user_type: String::from(session_user_type),
        }
    }
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
    let given_session_id = criteria.id.as_str();

    let session: Session = sessions.filter(sessions::id.eq(given_session_id)).first(connection)?;
  
    if session.is_conference() {
        return get_conference_people(connection, session.conference_id);
    }
   
    get_session_people(connection, given_session_id)
}

fn get_conference_people(connection: &MysqlConnection, conf_id: Option<String>) -> PeopleResult {

    let session_people: Vec<SessionPeople> = session_users
        .inner_join(users)
        .inner_join(sessions)
        .filter(conference_id.eq(conf_id.unwrap()))
        .load::<(SessionUser,User,Session)>(connection)?
        .iter()
        .map(|tuple| SessionPeople {
            session_user: tuple.0.clone(),
            user: tuple.1.clone(),
        })
        .collect();

    Ok(session_people)
}

fn get_session_people(connection: &MysqlConnection, given_session_id: &str) -> PeopleResult {
    
    let session_people: Vec<SessionPeople> = session_users
        .inner_join(users)
        .filter(session_id.eq(given_session_id))
        .load::<(SessionUser,User)>(connection)?
        .iter()
        .map(|tuple| SessionPeople {
            session_user: tuple.0.clone(),
            user: tuple.1.clone(),
        })
        .collect();

    Ok(session_people)
}
