// User is the first domain object that involves in almost all
// the future transactions of the platform.

// The users table houses all the users of the platform and shall be read
// along with the emails table

use crate::schema::users;
use crate::commons::util;

use chrono::NaiveDateTime;

// The Order of the fiels are very important 
#[derive(Queryable,Debug)]
pub struct User {
    pub id: i32,
    pub full_name: String,
    pub email: String,
    pub fuzzy_id: String,
    pub blocked: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Fields that we can safely expose to APIs
#[juniper::object(description = "Fields that we can safely expose to APIs")]
impl User {
    pub fn fuzzy_id(&self) -> &str {
        self.fuzzy_id.as_str()
    }

    pub fn email(&self) -> &str {
        self.email.as_str()
    }

    pub fn name(&self) -> &str {
        self.full_name.as_str()
    }
}

// Registration represents the fields we obtain from user 
// for Creating a new User in the system
#[derive(juniper::GraphQLInputObject)]
pub struct Registration {
    pub full_name: String,
    pub email: String,
}

// Fields we require to persist into User
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub full_name: String,
    pub email: String,
    pub fuzzy_id: String,
}

impl NewUser {
    pub fn from(registration: &Registration) -> NewUser {
        let fuzzy_id = util::fuzzy_id();

        NewUser {
            full_name: registration.full_name.to_owned(),
            email: registration.email.to_owned(),
            fuzzy_id: fuzzy_id,
        }
    }
}
