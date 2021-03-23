use diesel::prelude::*;
use super::prelude::*;

use crate::services::users::register;
use crate::services::users::reset_password;

use crate::models::users::Registration;
use crate::models::users::ResetPasswordRequest;

#[test]
pub fn should_reset_password() {

    let connection = connection_without_transaction();

    connection.test_transaction::<_,String,_>(||{

        let reg_request = build_registration_request();
        let reg_result = register(&connection,&reg_request);

        assert_eq!(reg_result.is_ok(),true);

        let request = build_reset_password_request();
        let result = reset_password(&connection, &request);

        assert_eq!(result.is_ok(),true);

        Ok(())
    });
}

/**
 * When we request a password reset with a wrong credential,
 * the service should reject the request
 */
#[test]
pub fn should_deny_if_wrong_request() {

    let connection = connection_without_transaction();

    connection.test_transaction::<_,String,_>(||{

        let reg_request = build_registration_request();
        let reg_result = register(&connection,&reg_request);

        assert_eq!(reg_result.is_ok(),true);

        let request = build_wrong_reset_password_request();
        let result = reset_password(&connection,&request);

        assert_eq!(result.is_err(),true);

        Ok(())
    });
}

fn build_reset_password_request() -> ResetPasswordRequest{

    ResetPasswordRequest{
        email:"email1@krscode.com".to_string(),
        new_password: "new_password".to_string(),
        password:"password".to_string()
    }
}

fn build_wrong_reset_password_request() -> ResetPasswordRequest{

    ResetPasswordRequest{
        email:"email1@krscode.com".to_string(),
        new_password: "new_password".to_string(),
        password:"wrong_password".to_string()
    }
}

/**
 * Used to precreate a user with a credential.
 */
fn build_registration_request() -> Registration {
    Registration {
        full_name: String::from("Full_Name-1"),
        email: String::from("email1@krscode.com"),
        password: String::from("password"),
    }
}