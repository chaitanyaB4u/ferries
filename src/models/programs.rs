/**
 * A coach can offer mutilple Programs. We define the structure of
 * the program. 
 */

use chrono::{NaiveDateTime};

use crate::schema::programs;
use crate::commons::chassis::{ValidationError};
use crate::commons::util;


/**
 * The structure represents One row of the programs table.
 */
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

/**
 * Let us expose certain limited elements of the structure to the outside world
 *
 */
#[juniper::object(description="The fields we offer to the Web-UI ")]
impl Program {

    pub fn fuzzy_id(&self) -> &str {
        self.fuzzy_id.as_str()
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

/**
 * We receive a request from a coach to create a New Program
 */
#[derive(juniper::GraphQLInputObject)]
pub struct NewProgramRequest {
    pub name: String,
    pub coach_id: i32,
    pub description: String
}

/**
 * We validate the NewProgramRequest to inform back, if we miss any important
 * fields before creating the Program.
 */
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

// The Persistable entity with the Fuzzy_id injected by us.
#[derive(Insertable)]
#[table_name = "programs"]
pub struct NewProgram {
    pub name: String,
    pub coach_id: i32,
    pub active: bool,
    pub fuzzy_id: String,
    pub description: String,
}

/**
 * Transforming the NewProgramRequest into NewProgram by inject a unique
 * fuyzz_id.
 */
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
