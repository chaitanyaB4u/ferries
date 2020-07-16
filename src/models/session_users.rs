use crate::commons::util;
use crate::schema::session_users;

use crate::models::sessions::Session;
use crate::models::users::{User,UserType};

#[derive(Queryable, Debug, Identifiable)]
pub struct SessionUser {
    pub id: i32,
    pub fuzzy_id: String,
    pub session_id: i32,
    pub user_id: i32,
    pub user_type: Option<String>,
}

#[derive(Insertable)]
#[table_name = "session_users"]
pub struct NewSessionUser {
    pub fuzzy_id: String,
    pub session_id: i32,
    pub user_id: i32,
    pub user_type: String,
}

impl NewSessionUser {

    pub fn into(session: &Session, user: &User, user_type: UserType) -> NewSessionUser {
        let fuzzy_id = util::fuzzy_id();
        
        NewSessionUser {
            fuzzy_id,
            session_id: session.id,
            user_id: user.id,
            user_type:String::from(user_type.as_str()),
        }
    }
}
