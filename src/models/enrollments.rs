use chrono::NaiveDateTime;

use crate::models::programs::Program;
use crate::models::users::User;

use crate::commons::chassis::ValidationError;
use crate::schema::enrollments;

#[derive(Queryable, Debug, Identifiable)]
pub struct Enrollment {
    pub id: i32,
    pub program_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub member_id: i32,
    pub fuzzy_id: String
}

#[juniper::object(description = "The fields we offer to the Web-UI ")]
impl Enrollment {
    pub fn id(&self) -> i32 {
        self.id
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct NewEnrollmentRequest {
    pub program_fuzzy_id: String,
    pub user_fuzzy_id: String,
}

impl NewEnrollmentRequest {
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors: Vec<ValidationError> = Vec::new();

        if self.program_fuzzy_id.trim().is_empty() {
            errors.push(ValidationError::new(
                "program_fuzzy_id",
                "The Program id is invalid.",
            ));
        }

        if self.user_fuzzy_id.trim().is_empty() {
            errors.push(ValidationError::new(
                "user_fuzzy_id",
                "The User id is invalid.",
            ));
        }

        errors
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct EnrollmentCriteria {
    pub program_fuzzy_id: String,
}

#[derive(Insertable)]
#[table_name = "enrollments"]
pub struct NewEnrollment {
    pub program_id: i32,
    pub member_id: i32,
}

impl NewEnrollment {
    pub fn from(program: &Program, user: &User) -> NewEnrollment {
        NewEnrollment {
            program_id: program.id,
            member_id: user.id,
        }
    }
}
