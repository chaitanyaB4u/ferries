use diesel::prelude::*;
use super::prelude::*;

use crate::models::users::Registration;
use crate::models::users::LoginRequest;

use crate::services::users::register;
use crate::services::users::authenticate;
use crate::services::users::INVALID_CREDENTIAL;

#[test]
pub fn should_authenticate_valid_user() {

    let connection = connection_without_transaction();

    connection.test_transaction::<_,String,_>(||{
        
        let reg_request = build_registration_request();
        let reg_result = register(&connection,&reg_request);

        assert_eq!(reg_result.is_ok(),true);

        let request = build_known_login_request();
        let result = authenticate(&connection, request);

        assert_eq!(result.is_ok(),true);

        Ok(())
    });
}

#[test]
pub fn should_defend_invalid_login() {
    let connection = connection_without_transaction();

    connection.test_transaction::<_,String,_>(||{

        let reg_request = build_registration_request();
        let reg_result = register(&connection,&reg_request);
        assert_eq!(reg_result.is_ok(),true);

        let request = build_invalid_login_request();
        let result  = authenticate(&connection, request);
        assert_eq!(result.is_err(),true);
        assert_eq!(result.unwrap_err(),INVALID_CREDENTIAL);

        Ok(())
    });
}

/**
 * Used to precreate a user with a credential.
 */
fn build_registration_request() -> Registration {
    Registration {
        full_name: String::from("Full_Name-1"),
        email: String::from("email3@krscode.com"),
        password: String::from("password"),
    }
}

/**
 * Mysql equal operation is case insensitive. Hence
 * email3@krscode.com should be equal to Email3@Krscode.com
 */
fn build_known_login_request() -> LoginRequest {
    LoginRequest {
        email: "Email3@Krscode.com".to_string(),
        password: "password".to_string(),
    }
}

/**
 * Let us slightly change the password
 */
fn build_invalid_login_request() -> LoginRequest {
    LoginRequest {
        email: "email3@krscode.com".to_string(),
        password: "password1".to_string(),
    }
}
