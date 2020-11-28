use crate::commons::chassis::ValidationError;
use crate::commons::util;
use crate::schema::base_programs;
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable, Associations,Debug)]
pub struct BaseProgram {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub genre_id: String,
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[juniper::object(description = "The fields we offer to outside world")]
impl BaseProgram {
    pub fn id(&self) -> &str {
        self.id.as_str()
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

    pub fn genreId(&self) -> &str {
        self.genre_id.as_str()
    }
}


/**
 * We receive a request from a coach to create a New Program
 */
#[derive(juniper::GraphQLInputObject)]
pub struct NewBaseProgramRequest {
    pub name: String,
    pub description: String,
    pub genre_id: String,
    pub coach_id: String,
}

/**
 * We validate the NewProgramRequest to inform back, if we miss any important
 * fields before creating the Program.
 */
impl NewBaseProgramRequest {
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors: Vec<ValidationError> = Vec::new();

        if self.name.trim().is_empty() {
            errors.push(ValidationError::new("name", "The name of the base program is a must."));
        }

        if self.description.trim().is_empty() {
            errors.push(ValidationError::new("description", "We need a description to start with."));
        }

        if self.coach_id.trim().is_empty() {
            errors.push(ValidationError::new("coach_id", "coach id is a must"));
        }

        if self.genre_id.trim().is_empty() {
            errors.push(ValidationError::new("genre_id", "The genre of the program is required."));
        }
        errors
    }
}

// The Persistable entity with the Fuzzy_id injected by us.
#[derive(Insertable)]
#[table_name = "base_programs"]
pub struct NewBaseProgram {
    pub id: String,
    pub name: String,
    pub description: String,
    pub genre_id: String,
    pub active: bool,
}

impl NewBaseProgram {
    pub fn from(request: &NewBaseProgramRequest) -> NewBaseProgram {
        let fuzzy_id = util::fuzzy_id();

        NewBaseProgram {
            id: fuzzy_id,
            name: request.name.to_owned(),
            description: request.description.to_owned(),
            genre_id: request.genre_id.to_owned(),
            active: false,
        }
    }
}
