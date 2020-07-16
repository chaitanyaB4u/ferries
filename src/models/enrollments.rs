use chrono::{NaiveDateTime};

use crate::schema::enrollments;
use crate::commons::chassis::{ValidationError};

#[derive(Queryable,Debug)]
pub struct Enrollment {
    pub id: i32,
    pub program_id: i32,
    pub team_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[juniper::object(description="The fields we offer to the Web-UI ")]
impl Enrollment {

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn program_id(&self) -> i32 {
        self.program_id
    }

    pub fn team_id(&self) -> i32 {
        self.team_id
    }
}

#[derive(juniper::GraphQLInputObject)]
#[derive(Insertable)]
#[table_name = "enrollments"]
pub struct NewEnrollmentRequest {
    pub program_id: i32,
    pub team_id: i32,
}

impl NewEnrollmentRequest {

    pub fn validate(&self) ->Vec<ValidationError> {

        let mut errors: Vec<ValidationError> = Vec::new();

        if self.program_id <= 0 {
            errors.push(ValidationError::new("program_id", "Program id is invalid."));
        }

        if self.team_id <= 0 {
            errors.push(ValidationError::new("team_id", "Team member id is invalid."));
        }

        errors
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct EnrollmentCriteria {
    pub program_fuzzy_id: String,
}