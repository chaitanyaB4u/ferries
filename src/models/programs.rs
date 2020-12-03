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
    pub genre_id: Option<String>,
    pub is_parent: bool,
    pub parent_program_id: Option<String>,
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

    pub fn genre_id(&self) -> &Option<String> {
        &self.genre_id
    }

    pub fn parent_program_id(&self) -> &str {
       self.coalesce_parent_id()
    }

    pub fn is_parent(&self) -> bool {
        self.is_parent
    }
}

impl Program {

    pub fn coalesce_parent_id(&self) -> &str {
        match &self.parent_program_id {
            None => &self.id,
            Some(value) => &value
        }
    }
}

/**
 * 1. We receive a request from a coach to create a New Program.
 *
 * 2. We create this request internally from base_program after
 * instantiating and associating a program to a coach.
 *
 */
#[derive(juniper::GraphQLInputObject)]
pub struct NewProgramRequest {
    pub name: String,
    pub coach_id: String,
    pub description: String,
    pub is_private: bool,
    pub genre_id: Option<String>,
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
    pub is_parent: bool,
    pub parent_program_id: String,
    pub genre_id: Option<String>,
}

/**
 * Transforming the NewProgramRequest into NewProgram by injecting
 * a unique fuyzz_id.
 */
impl NewProgram {
    /**
     * The parent program will have the same id as base_program_id
     */
    pub fn from_request(request: &NewProgramRequest, coach: &Coach) -> NewProgram {
        let fuzzy_id = util::fuzzy_id();

        NewProgram {
            id: fuzzy_id.to_owned(),
            parent_program_id: fuzzy_id,
            is_parent: true,
            name: request.name.to_owned(),
            description: request.description.to_owned(),
            is_private: request.is_private,
            active: false,
            coach_name: coach.full_name.to_owned(),
            coach_id: coach.id.to_owned(),
            genre_id: request.genre_id.to_owned(),
        }
    }

    /**
     * We spawn a program while attaching another coach to the parent program
     */
    pub fn from_parent_program(parent_program: &Program, coach: &Coach) -> NewProgram {
        let fuzzy_id = util::fuzzy_id();

        NewProgram {
            id: fuzzy_id,
            parent_program_id: parent_program.id.to_owned(),
            is_parent: false,
            name: parent_program.name.to_owned(),
            description: String::from("-"),
            is_private: true,
            active: parent_program.active,
            coach_name: coach.full_name.to_owned(),
            coach_id: coach.id.to_owned(),
            genre_id: parent_program.genre_id.to_owned(),
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


#[derive(juniper::GraphQLInputObject)]
pub struct AssociateCoachRequest {
    pub peer_coach_email: String,
    pub program_id: String,
    pub admin_coach_id: String,
}


pub struct ProgramCoach {
    pub program:Program,
    pub coach:Coach,
}

#[juniper::object(description="To offer the List of all the PeerCoaches of a Program")]
impl ProgramCoach {
    pub fn program(&self) -> &Program {
        &self.program
    }
    pub fn coach(&self) -> &Coach {
        &self.coach
    }
}
