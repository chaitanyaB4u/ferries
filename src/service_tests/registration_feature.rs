use diesel::prelude::*;
use super::prelude::*;

use crate::models::users::{User,Registration};
use crate::models::ferror::Ferror;

use crate::services::users::register;
use crate::services::users::{REGISTERED_ALREADY};


#[test]
pub fn should_register_valid_user() {
    let connection = connection_without_transaction();

    connection.test_transaction::<_,Ferror,_>(|| {

        let request = build_registration_request();

        let result: Result<User,Ferror> = register(&connection, &request);
        assert_eq!(result.is_ok(),true);

        let user: User = result.unwrap();
        assert_eq!(user.email,request.email);
        assert_eq!(user.full_name,request.full_name);

        Ok(())
    });
}

#[test]
pub fn should_defend_duplicate_registration() {
    let connection = connection_without_transaction();

    connection.test_transaction::<_,Ferror,_>(|| {

        let request = build_registration_request();

        let result: Result<User,Ferror> = register(&connection, &request);
        assert_eq!(result.is_ok(),true);
        
        let result: Result<User,Ferror> = register(&connection, &request);
        assert_eq!(result.is_ok(),false);

        let ferror: Ferror = result.unwrap_err();
        let error = ferror.errors.get(0).unwrap();
        assert_eq!(error.message,REGISTERED_ALREADY);

        Ok(())
    });
}

#[test]
pub fn should_reject_registration_with_blank_fields() {
    let connection = connection_without_transaction();

    connection.test_transaction::<_,Ferror,_>(||{
        let result = register(&connection, &build_blank_registration_request());
        assert_eq!(result.is_err(), true);

        let ferror: Ferror = result.unwrap_err();
        assert_eq!(ferror.errors.len(),3);
    
        Ok(())
    });
}

fn build_registration_request() -> Registration {

    Registration {
        full_name: String::from("Full_Name-1"),
        email: String::from("email_reg@krscode.com"),
        password: String::from("password"),
    }
}

fn build_blank_registration_request() -> Registration {

    Registration {
        full_name: String::from(""),
        email: String::from(""),
        password: String::from(""),
    }
}

