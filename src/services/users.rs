use crate::models::users::{NewUser, Registration, User};
use crate::schema::users::dsl::*;
use diesel::prelude::*;

const REGISTERED_ALREADY: &'static str = "It seems you have already registered with us.";
const BLANK_EMAIL: &'static str = "The email id is required.";
const BLANK_FULL_NAME: &'static str = "Your full name is required.";

pub fn get_users(connection: &MysqlConnection) -> Vec<User> {
    users
        .limit(100)
        .load::<User>(connection)
        .expect("Error Fetching users")
}

pub fn register(connection: &MysqlConnection,registration: &Registration) -> Result<User, &'static str> {
    
    let result = find_by_email(connection, registration.email.as_str());
    if result.is_ok() {
        return Err(REGISTERED_ALREADY);
    }

    if registration.email.trim().len() == 0 {
        return Err(BLANK_EMAIL);
    }

    if registration.full_name.trim().len() == 0 {
        return Err(BLANK_FULL_NAME);
    }

    let result = create_user(connection, registration);

    match result {
        Ok(user) => Ok(user),
        Err(_) => {
            return Err("Unable to Create User");
        }
    }
}

pub fn find_by_fuzzy_id(connection: &MysqlConnection, fuzzy: &str) -> QueryResult<User> {
    users.filter(fuzzy_id.eq(fuzzy)).first(connection)
}

fn find_by_email(connection: &MysqlConnection, email_str: &str) -> QueryResult<User> {
    users.filter(email.eq(email_str)).first(connection)
}

fn create_user(connection: &MysqlConnection, registration: &Registration) -> QueryResult<User> {
    let new_user = NewUser::from(registration);
    
    diesel::insert_into(users)
        .values(&new_user)
        .execute(connection)
        .expect("Error while creating the User");

    find_by_fuzzy_id(connection, new_user.fuzzy_id.as_str())
}
