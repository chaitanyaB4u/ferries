use crate::schema::options;
use crate::commons::chassis::{ValidationError};
use crate::commons::util;

use chrono::{NaiveDateTime};

#[derive(Queryable,Debug)]
pub struct Constraint {
    pub id:String,
    pub enrollment_id:String,
    pub description : Option<String>,
    pub created_at : NaiveDateTime,
    pub updated_at : NaiveDateTime
}

#[juniper::object]
impl Constraint {

    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn enrollment_id(&self) -> &str {
        self.enrollment_id.as_str()
    }

    pub fn description(&self) -> &str {
        let value: &str = match &self.description {
            None=>"_",
            Some(value)=>value.as_str()
        };
        value
    }

    pub fn createdAt(&self) -> NaiveDateTime {
        self.created_at
    }

}

#[derive(juniper::GraphQLInputObject)]
pub struct NewOptionRequest {
    pub enrollment_id: String,
    pub description: String
}

impl NewOptionRequest {
    
    pub fn validate(&self) -> Vec<ValidationError> {

        let mut errors: Vec<ValidationError> = Vec::new();

        if self.enrollment_id.trim().is_empty(){
            errors.push(ValidationError::new("enrollment_id","Enrollment Id is a must."));
        }

        errors
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct UpdateOptionRequest {
    pub id: String,
    pub description: String
}

impl UpdateOptionRequest {
    pub fn validate(&self) -> Vec<ValidationError> {

        let mut errors: Vec<ValidationError> = Vec::new();

        if self.id.trim().is_empty(){
            errors.push(ValidationError::new("id","Id is a must."));
        }

        errors
    }
}

#[derive(Insertable)]
#[table_name = "options"]
pub struct NewOption {
    pub id:String,
    pub enrollment_id:String,
    pub description: String,
}

impl NewOption  {

    pub fn from(request: &NewOptionRequest) -> NewOption {
        let fuzzy_id = util::fuzzy_id();

        let new_option = NewOption {
            id:fuzzy_id,
            enrollment_id:request.enrollment_id.to_owned(),
            description: request.description.to_owned()
        };

        new_option
    }
}