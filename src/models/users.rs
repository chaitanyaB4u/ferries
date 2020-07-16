// User is the first domain object that involves in almost all
// the future transactions of the platform.
// The users table houses all the users of this platform.

use chrono::NaiveDateTime;

use crate::schema::users;
use crate::commons::util;




// The Order of the fiels are very important
// The User struct is purely for internal consumption. 
// See the Juniper:object for the fields we exposed to outside 
#[derive(Queryable,Debug,Identifiable)]
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
#[juniper::object(description = "The exposed attributes of the User Structure.")]
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
// while Creating a new User in the system
#[derive(juniper::GraphQLInputObject)]
pub struct Registration {
    pub full_name: String,
    pub email: String,
}

// Fields we require to persist into the users table
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub full_name: String,
    pub email: String,
    pub fuzzy_id: String,
}

// A way to transform the inbound registration request into the persistable
// NewUser structure.

// Let us generate the fuzzy_id, so that we can use it to find and return 
// the NewUser structure to the requester, post-creation.
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


const COACH: &'static str = "coach";
const MEMBER: &'static str = "member";

pub enum UserType {
    COACH, 
    MEMBER
}

impl UserType {
    
    pub fn as_str(&self) -> &'static str {

        match self {
            UserType::COACH => COACH,
            UserType::MEMBER => MEMBER
        }

    }
}
