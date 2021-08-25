use diesel::prelude::*;
use super::prelude::connection_without_transaction;

use crate::services::programs::create_new_program;

use crate::services::users::INVALID_COACH_ID;

use crate::models::programs::NewProgramRequest;

#[test]
pub fn should_not_create_program_if_unknown_coach() {

    let connection = connection_without_transaction();

    connection.test_transaction::<_,String,_>(||{
        let request = build_program_with_unknown_coach();
        
        let result = create_new_program(&connection, &request);

        assert_eq!(result.is_err(),true);
        
        let error = result.unwrap_err();
        
        assert_eq!(error,INVALID_COACH_ID);

        Ok(())
    });

}

fn build_program_request() -> NewProgramRequest {
    NewProgramRequest{
        coach_id:"coach-1".to_string(),
        name: "name-1".to_string(),
        description: "desc".to_string(),
        genre_id: None,
        is_private: true,
    }
}

fn build_program_with_unknown_coach() -> NewProgramRequest {
    NewProgramRequest{
        coach_id:"coach-x".to_string(),
        name: "name-1".to_string(),
        description: "desc".to_string(),
        genre_id: None,
        is_private: true,
    }
}