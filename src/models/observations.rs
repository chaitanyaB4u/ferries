use crate::schema::observations;
use crate::commons::chassis::{ValidationError};
use crate::commons::util;

use chrono::{NaiveDateTime};

#[derive(Queryable,Debug,Identifiable)]
pub struct Observation {
    pub id:String,
    pub enrollment_id:String,
    pub description : Option<String>,
    pub created_at : NaiveDateTime,
    pub updated_at : NaiveDateTime
}

#[juniper::object]
impl Observation {

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
pub struct NewObservationRequest {
    pub enrollment_id: String,
    pub description: String
}

impl NewObservationRequest {
    
    pub fn validate(&self) -> Vec<ValidationError> {

        let mut errors: Vec<ValidationError> = Vec::new();

        if self.enrollment_id.trim().is_empty(){
            errors.push(ValidationError::new("enrollment_id","Enrollment Id is a must."));
        }

        errors
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct UpdateObservationRequest {
    pub id: String,
    pub description: String
}

impl UpdateObservationRequest {
    pub fn validate(&self) -> Vec<ValidationError> {

        let mut errors: Vec<ValidationError> = Vec::new();

        if self.id.trim().is_empty(){
            errors.push(ValidationError::new("id","Id is a must."));
        }

        errors
    }
}

#[derive(Insertable)]
#[table_name = "observations"]
pub struct NewObservation {
    pub id:String,
    pub enrollment_id:String,
    pub description: String,
}

impl NewObservation  {

    pub fn from(request: &NewObservationRequest) -> NewObservation {
        let fuzzy_id = util::fuzzy_id();

        let new_observation = NewObservation {
            id:fuzzy_id,
            enrollment_id:request.enrollment_id.to_owned(),
            description: request.description.to_owned()
        };

        new_observation
    }
}