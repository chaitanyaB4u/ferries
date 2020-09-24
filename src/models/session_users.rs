use crate::commons::util;
use crate::schema::session_users;

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
