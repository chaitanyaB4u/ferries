use crate::schema::programs;
use crate::commons::chassis::{ValidationError};
use crate::commons::util;

use chrono::{NaiveDateTime};

#[derive(Queryable,Debug)]
pub struct Program {
    pub id: i32,
    pub name: String,
    pub coach_id: i32,
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub fuzzy_id: String,
    pub description: Option<String>,
}

#[juniper::object(description = "Fields that we can safely expose to APIs")]
impl Program {

    pub fn fuzzy_id(&self) -> &str {
        self.fuzzy_id.as_str()
    }

    pub fn coach_id(&self) -> i32 {
        self.coach_id
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> &str {
        match &self.description {
            None=>"_",
            Some(value)=>value.as_str()
        }
    }
}


#[derive(juniper::GraphQLInputObject)]
pub struct NewProgramRequest {
    pub name: String,
    pub coach_id: i32,
    pub description: String
}

impl NewProgramRequest {

    pub fn validate(&self) ->Vec<ValidationError> {

        let mut errors: Vec<ValidationError> = Vec::new();

        if self.name.trim().is_empty() {
            errors.push(ValidationError::new("name", "name of the program is a must."));
        }

        if self.coach_id <= 0 {
            errors.push(ValidationError::new("coach_id", "coach is invalid."));
        }

        if self.description.trim().is_empty() {
            errors.push(ValidationError::new("desciption", "description of the program is a must."));
        }

        errors

    }

}

// The Persistable entity
#[derive(Insertable)]
#[table_name = "programs"]
pub struct NewProgram {
    pub name: String,
    pub coach_id: i32,
    pub active: bool,
    pub fuzzy_id: String,
    pub description: String,
}

impl NewProgram {

    pub fn from(request: &NewProgramRequest) -> NewProgram {

        let fuzzy_id = util::fuzzy_id();

        NewProgram {
            name:request.name.to_owned(),
            coach_id:request.coach_id,
            active:true,
            fuzzy_id:fuzzy_id,
            description:request.description.to_owned()
        }
        
    }
}
