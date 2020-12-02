use diesel::prelude::*;

use crate::commons::util;
use crate::models::coaches::Coach;
use crate::models::users::{LoginRequest, NewUser, Registration, ResetPasswordRequest, User};

use crate::schema::users;
use crate::schema::users::dsl::*;

use crate::schema::coaches;
use crate::schema::coaches::dsl::*;

const REGISTERED_ALREADY: &str = "It seems you have already registered with us.";
const BLANK_EMAIL: &str = "The email id is required.";
const BLANK_FULL_NAME: &str = "Your full name is required.";
const INVALID_USER_ID: &str = "Invalid User Id";
const CREATION_ERROR: &str = "Unable to create a new user";
const INVALID_CREDENTIAL: &str = "Invalid Credential";
const PASSWORD_RESET_FAILED: &str = "Failed to reset the password.";
const INVALID_COACH_EMAIL: &str = "Invalid Coach email address";
const INVALID_COACH_ID: &str = "Invalid Coach Id";

pub fn register(connection: &MysqlConnection, registration: &Registration) -> Result<User, &'static str> {
    if registration.email.trim().is_empty() {
        return Err(BLANK_EMAIL);
    }

    if registration.full_name.trim().is_empty() {
        return Err(BLANK_FULL_NAME);
    }

    is_registered(connection, registration.email.as_str())?;

    create_user(connection, registration)
}

pub fn authenticate(connection: &MysqlConnection, request: LoginRequest) -> Result<User, &'static str> {
    let result: QueryResult<String> = users.filter(users::email.eq(request.email.as_str().trim())).select(password).first(connection);
    if result.is_err() {
        return Err(INVALID_CREDENTIAL);
    }

    let flag = util::is_equal(result.unwrap().as_str(), request.password.as_str());
    if !flag {
        return Err(INVALID_CREDENTIAL);
    }

    let result: QueryResult<User> = users.filter(users::email.eq(request.email.as_str().trim())).first(connection);
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

    let result = diesel::update(users).filter(users::email.eq(user.email.as_str())).set(password.eq(hashed_password)).execute(connection);

    if result.is_err() {
        return Err(PASSWORD_RESET_FAILED);
    }

    Ok(user)
}

pub fn find(connection: &MysqlConnection, the_id: &str) -> Result<User, &'static str> {
    
    let result = users.filter(users::id.eq(the_id)).first(connection);

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

fn is_registered(connection: &MysqlConnection, email_str: &str) -> Result<bool, &'static str> {
    let result: QueryResult<User> = users.filter(users::email.eq(email_str)).first(connection);

    if result.is_ok() {
        return Err(REGISTERED_ALREADY);
    }

    Ok(false)
}

/**
 * The id of coach and user_id will be the same. The Coaches table is a
 * convenience for avoiding self-join.
 */
pub fn find_coach_by_id(connection: &MysqlConnection, the_coach_id: &str) -> Result<Coach, &'static str> {

    let coach_result = coaches.filter(coaches::id.eq(the_coach_id)).first(connection);

    if coach_result.is_err() {
        return Err(INVALID_COACH_ID);
    }

    Ok(coach_result.unwrap())
}

pub fn find_coach_by_email(connection: &MysqlConnection, peer_coach_email: &str) -> Result<Coach, &'static str> {

    let coach_result = coaches.filter(coaches::email.eq(peer_coach_email)).first(connection);

    if coach_result.is_err() {
        return Err(INVALID_COACH_EMAIL);
    }

    Ok(coach_result.unwrap())
}
