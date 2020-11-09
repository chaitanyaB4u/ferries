use diesel::prelude::*;

use crate::commons::util;
use crate::models::users::{LoginRequest, NewUser, Registration, ResetPasswordRequest, User};
use crate::schema::users::dsl::*;

const REGISTERED_ALREADY: &'static str = "It seems you have already registered with us.";
const BLANK_EMAIL: &'static str = "The email id is required.";
const BLANK_FULL_NAME: &'static str = "Your full name is required.";
const INVALID_USER_ID: &'static str = "Invalid User Id";
const CREATION_ERROR: &'static str = "Unable to create a new user";
const INVALID_CREDENTIAL: &'static str = "Invalid Credential";
const PASSWORD_RESET_FAILED: &'static str = "Failed to reset the password.";

pub fn register(connection: &MysqlConnection, registration: &Registration) -> Result<User, &'static str> {
    if registration.email.trim().is_empty() {
        return Err(BLANK_EMAIL);
    }

    if registration.full_name.trim().is_empty() {
        return Err(BLANK_FULL_NAME);
    }

    find_by_email(connection, registration.email.as_str())?;

    create_user(connection, registration)
}

pub fn authenticate(connection: &MysqlConnection, request: LoginRequest) -> Result<User, &'static str> {
    let result: QueryResult<String> = users.filter(email.eq(request.email.as_str())).select(password).first(connection);
    if result.is_err() {
        return Err(INVALID_CREDENTIAL);
    }

    let flag = util::is_equal(result.unwrap().as_str(), request.password.as_str());
    if !flag {
        return Err(INVALID_CREDENTIAL);
    }

    let result: QueryResult<User> = users.filter(email.eq(request.email.as_str())).first(connection);
    if result.is_err() {
        return Err(INVALID_CREDENTIAL);
    }

    Ok(result.unwrap())
}

pub fn reset_password(connection: &MysqlConnection, request: &ResetPasswordRequest) -> Result<User, &'static str> {
    let login_request = LoginRequest {
        email: request.email.to_owned(),
        password: request.password.to_owned(),
    };
    let user = authenticate(connection, login_request)?;

    let hashed_password = util::hash(request.new_password.as_str());

    let result = diesel::update(users).filter(email.eq(user.email.as_str())).set(password.eq(hashed_password)).execute(connection);

    if result.is_err() {
        return Err(PASSWORD_RESET_FAILED);
    }

    Ok(user)
}

pub fn find(connection: &MysqlConnection, the_id: &str) -> Result<User, &'static str> {
    let result = users.filter(id.eq(the_id)).first(connection);

    if result.is_err() {
        return Err(INVALID_USER_ID);
    }

    Ok(result.unwrap())
}


fn create_user(connection: &MysqlConnection, registration: &Registration) -> Result<User, &'static str> {
    let new_user = NewUser::from(registration);

    let result = diesel::insert_into(users).values(&new_user).execute(connection);

    if result.is_err() {
        return Err(CREATION_ERROR);
    }

    find(connection, new_user.id.as_str())
}

fn find_by_email(connection: &MysqlConnection, email_str: &str) -> Result<bool, &'static str> {
    let result: QueryResult<User> = users.filter(email.eq(email_str)).first(connection);

    if result.is_ok() {
        return Err(REGISTERED_ALREADY);
    }

    Ok(true)
}
