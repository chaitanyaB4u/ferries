/**
 * Important: The Mutation Result might seem like a Code Duplication, 
 * but is unavoidable.
 * 
 * Excerpt from Graphql:Rust - Objects and Generics
 * 
 * Yet another point where GraphQL and Rust differs is in how generics work. 
 * In Rust, almost any type could be generic - that is, take type parameters. 
 * In GraphQL, there are only two generic types: lists and non-nullables.
 * This poses a restriction on what you can expose in GraphQL from Rust: 
 * no generic structs can be exposed - all type parameters must be bound. 
 * For example, you can not make e.g. Result<T, E> into a GraphQL type, 
 * but you can make e.g. Result<User, String> into a GraphQL type.
 */
 
use crate::models::sessions::{Session};
use crate::models::programs::{Program};
use crate::models::enrollments::{Enrollment};


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


#[juniper::object(name = "ProgramResult")]
impl MutationResult<Program> {
    pub fn program(&self) -> Option<&Program> {
        self.0.as_ref().ok()
    }

    pub fn error(&self) -> Option<&Vec<ValidationError>> {
        self.0.as_ref().err()
    }
}

#[juniper::object(name = "EnrollmentResult")]
impl MutationResult<Enrollment> {
    pub fn enrollment(&self) -> Option<&Enrollment> {
        self.0.as_ref().ok()
    }

    pub fn error(&self) -> Option<&Vec<ValidationError>> {
        self.0.as_ref().err()
    }
}


pub fn service_error<T>(message: &str) -> MutationResult<T> {
    let mut v: Vec<ValidationError> = Vec::new();
    let ve = ValidationError{field: String::from("service"),message: String::from(message)};
    v.push(ve);
    MutationResult(Err(v))
}