use crate::models::sessions::{Session};
use crate::models::programs::{Program};

#[derive(juniper::GraphQLObject)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(field: &str, message: &str) -> ValidationError{
        ValidationError{field:String::from(field),message:String::from(message)}   
    }
}
pub struct MutationResult<T>(pub Result<T, Vec<ValidationError>>);

#[juniper::object(name = "SessionResult")]
impl MutationResult<Session> {
    pub fn session(&self) -> Option<&Session> {
        self.0.as_ref().ok()
    }

    pub fn error(&self) -> Option<&Vec<ValidationError>> {
        self.0.as_ref().err()
    }
}

pub fn session_service_error(message: &str) -> MutationResult<Session> {
    let mut v: Vec<ValidationError> = Vec::new();
    let ve = ValidationError{field: String::from("service"),message: String::from(message)};
    v.push(ve);
    MutationResult(Err(v))
}

#[juniper::object(name = "ProgramResult")]
impl MutationResult<Program> {
    pub fn program(&self) -> Option<&Program> {
        self.0.as_ref().ok()
    }

    pub fn error(&self) -> Option<&Vec<ValidationError>> {
        self.0.as_ref().err()
    }
}

pub fn program_service_error(message: &str) -> MutationResult<Program> {
    let mut v: Vec<ValidationError> = Vec::new();
    let ve = ValidationError{field: String::from("service"),message: String::from(message)};
    v.push(ve);
    MutationResult(Err(v))
}