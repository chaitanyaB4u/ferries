use chrono::NaiveDateTime;

use crate::models::programs::Program;
use crate::models::users::User;

use crate::commons::chassis::ValidationError;
use crate::schema::enrollments;

#[derive(Queryable, Debug, Identifiable)]
pub struct Enrollment {
    pub id: String,
    pub program_id: String,
    pub member_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[juniper::object(description = "The fields we offer to the Web-UI ")]
impl Enrollment {
    pub fn id(&self) -> &str {
        self.id.as_str()
    }
    pub fn program_id(&self) -> &str {
        self.program_id.as_str()
    }
    pub fn member_id(&self) -> &str {
        self.member_id.as_str()
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct NewEnrollmentRequest {
    pub program_id: String,
    pub user_id: String,
}

impl NewEnrollmentRequest {
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors: Vec<ValidationError> = Vec::new();

        if self.program_id.trim().is_empty() {
            errors.push(ValidationError::new(
                "program_id",
                "The Program id is invalid.",
            ));
        }

        if self.user_id.trim().is_empty() {
            errors.push(ValidationError::new(
                "user_id",
                "The User id is invalid.",
            ));
        }

        errors
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct EnrollmentCriteria {
    pub program_id: String,
}

#[derive(Insertable)]
#[table_name = "enrollments"]
pub struct NewEnrollment {
    pub program_id: String,
    pub member_id: String,
}

impl NewEnrollment {
    pub fn from(program: &Program, user: &User) -> NewEnrollment {
        NewEnrollment {
            program_id: program.id.to_owned(),
            member_id: user.id.to_owned(),
        }
    }
}
