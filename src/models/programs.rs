/**
 * A coach can offer mutilple Programs. We define the structure of
 * the program.
 */
use chrono::NaiveDateTime;

use crate::commons::chassis::ValidationError;
use crate::commons::util;
use crate::models::coaches::Coach;
use crate::schema::programs;

/**
 * The structure represents One row of the programs table.
 */
#[derive(Queryable, Debug, Identifiable, Associations)]
pub struct Program {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub active: bool,
    pub coach_name: String,
    pub coach_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_private: bool,
    pub base_program_id: Option<String>,
}

/**
 * Let us expose certain limited elements of the structure to the outside world
 *
 */
#[juniper::object(description = "The fields we offer to the Web-UI ")]
impl Program {
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> &str {
        match &self.description {
            None => "_",
            Some(value) => value.as_str(),
        }
    }

    pub fn coach_name(&self) -> &str {
        self.coach_name.as_str()
    }

    pub fn coach_id(&self) -> &str {
        self.coach_id.as_str()
    }

    pub fn is_private(&self) -> bool {
        self.is_private
    }
}

/**
 * We receive a request from a coach to create a New Program
 */
#[derive(juniper::GraphQLInputObject)]
pub struct NewProgramRequest {
    pub name: String,
    pub coach_id: String,
    pub description: String,
    pub is_private: bool,
    pub base_program_id: Option<String>,
}

/**
 * We validate the NewProgramRequest to inform back, if we miss any important
 * fields before creating the Program.
 */
impl NewProgramRequest {
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors: Vec<ValidationError> = Vec::new();

        if self.name.trim().is_empty() {
            errors.push(ValidationError::new("name", "name of the program is a must."));
        }

        if self.coach_id.trim().is_empty() {
            errors.push(ValidationError::new("coach_id", "coach id is a must"));
        }

        if self.description.trim().is_empty() {
            errors.push(ValidationError::new("description", "description of the program is a must."));
        }

        errors
    }
}

// The Persistable entity with the Fuzzy_id injected by us.
#[derive(Insertable)]
#[table_name = "programs"]
pub struct NewProgram {
    pub id: String,
    pub name: String,
    pub description: String,
    pub active: bool,
    pub is_private: bool,
    pub coach_name: String,
    pub coach_id: String,
    pub base_program_id: Option<String>,
}

/**
 * Transforming the NewProgramRequest into NewProgram by inject a unique
 * fuyzz_id.
 */
impl NewProgram {
    pub fn from(request: &NewProgramRequest, coach: &Coach) -> NewProgram {
        let fuzzy_id = util::fuzzy_id();

        NewProgram {
            id: fuzzy_id,
            name: request.name.to_owned(),
            description: request.description.to_owned(),
            is_private: request.is_private,
            active: false,
            coach_name: coach.full_name.to_owned(),
            coach_id: coach.id.to_owned(),
            base_program_id: request.base_program_id.to_owned()
        }
    }
}

#[derive(juniper::GraphQLEnum, PartialEq)]
pub enum ProgramTargetState {
    ACTIVATE,
    DEACTIVATE,
}

#[derive(juniper::GraphQLInputObject)]
pub struct ChangeProgramStateRequest {
    pub id: String,
    pub target_state: ProgramTargetState,
}
