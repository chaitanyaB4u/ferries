
use diesel::prelude::*;
use super::prelude::*;

use crate::models::users::{Registration};
use crate::models::programs::{NewProgramRequest};
use crate::models::sessions::{NewSessionRequest};

use crate::services::users::{register};
use crate::services::sessions::{create_session};

fn program_request(coach_id: String) -> NewProgramRequest {

    NewProgramRequest{
        name: String::from("Program-1"),
        coach_id: coach_id,
        description: String::from("Prog Description"),
        is_private: false,
        genre_id: None,
    }
}
fn session_request() -> NewSessionRequest{
    NewSessionRequest{
        program_id:String::from("1"),
        member_id:String::from("1"),
        name: String::from("name"),
        description: String::from("name"),
        duration: 14,
        start_time: String::from("12"),
    }
}