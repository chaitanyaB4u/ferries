/**
 * Ferris Error Container. 
 * When we deal with form fields and when we persist them into database
 * we may encounter validation errors. 
 * Ferror is to hold the vector of validation errors.
 */

use crate::commons::chassis::ValidationError;

#[derive(Debug)]
pub struct Ferror {
    pub errors: Vec<ValidationError>
}

impl From<&str> for Ferror {
    fn from(an_error:&str) -> Self {
        let ve = ValidationError::new("rule",an_error);
        Ferror{errors:vec![ve]}
    }
}

impl Ferror {
    pub fn new() -> Ferror {
        Ferror {
            errors: Vec::new()
        }
    }
    pub fn push(&mut self,field:&str,message: &str) {
        self.errors.push(ValidationError::new(field,message));
    }
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
}

impl Default for Ferror {
    fn default() -> Self {
        Self::new()
    }
}
