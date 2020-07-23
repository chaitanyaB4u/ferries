use crate::models::users::{NewUser, Registration, User,LoginRequest};
use crate::schema::users::dsl::*;
use diesel::prelude::*;


const REGISTERED_ALREADY: &'static str = "It seems you have already registered with us.";
const BLANK_EMAIL: &'static str = "The email id is required.";
const BLANK_FULL_NAME: &'static str = "Your full name is required.";
const INVALID_USER_FUZZY_ID: &'static str = "Invalid User Fuzzy Id";
const CREATION_ERROR: &'static str = "Unable to create a new user";

pub fn register(connection: &MysqlConnection,registration: &Registration) -> Result<User, &'static str> {
   
    if registration.email.trim().is_empty() {
        return Err(BLANK_EMAIL);
    }

    if registration.full_name.trim().is_empty() {
        return Err(BLANK_FULL_NAME);
    }

    find_by_email(connection, registration.email.as_str())?;
 
    create_user(connection, registration)
}

pub fn authenticate(connection: &MysqlConnection, request:LoginRequest) -> QueryResult<User> {
    users.filter(email.eq(request.email)).first(connection)
}

pub fn find_by_fuzzy_id(connection: &MysqlConnection, fuzzy: &str) -> Result<User,&'static str> {
    let result = users.filter(fuzzy_id.eq(fuzzy)).first(connection);

    if result.is_err() {
        return Err(INVALID_USER_FUZZY_ID);
    }

    Ok(result.unwrap())
}

pub fn get_users(connection: &MysqlConnection) -> Vec<User> {
    users
        .limit(100)
        .load::<User>(connection)
        .expect("Error Fetching users")
}


fn create_user(connection: &MysqlConnection, registration: &Registration) -> Result<User, &'static str> {
    let new_user = NewUser::from(registration);
    
    let result = diesel::insert_into(users).values(&new_user).execute(connection);

    if result.is_err() {
        return Err(CREATION_ERROR);
    }
      
    find_by_fuzzy_id(connection, new_user.fuzzy_id.as_str())
}

fn find_by_email(connection: &MysqlConnection, email_str: &str) -> Result<bool, &'static str> {
    let result: QueryResult<User> = users.filter(email.eq(email_str)).first(connection);
    
    if result.is_ok() {
        return Err(REGISTERED_ALREADY);
    }

    Ok(true)
}