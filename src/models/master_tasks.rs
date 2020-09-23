use crate::commons::chassis::ValidationError;
use crate::commons::util;
use crate::schema::master_tasks;

use chrono::{NaiveDateTime};

#[derive(Queryable, Debug, Identifiable)]
pub struct MasterTask {
    pub id : String,
    pub master_plan_id : String,
    pub abstract_task_id : String,
    pub duration : i32,
    pub min : i32,
    pub max : i32,
    pub task_type : String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub coach_id : String,
    pub role_id : String,
    pub coordinates : String
}

#[derive(juniper::GraphQLEnum)]
enum TaskType {
    START,
    STOP,
    DECISION,
    ACTIVITY,
    BUFFER,
    PROCEDURE
}

#[juniper::object]
impl MasterTask {
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn master_plan_id(&self) -> &str {
        self.master_plan_id.as_str()
    }

    pub fn abstract_task_id(&self) -> &str {
        self.abstract_task_id.as_str()
    }

    pub fn duration(&self) -> i32 {
        self.duration
    }

    pub fn min(&self) -> i32 {
        self.min
    }

    pub fn max(&self) -> i32 {
        self.max
    }

    pub fn createdAt(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn coach_id(&self) -> &str {
        self.coach_id.as_str()
    }

    pub fn role_id(&self) -> &str {
        self.role_id.as_str()
    }

    pub fn task_type(&self) -> &str {
        self.task_type.as_str()
    }

    pub fn coordinates(&self) -> &str {
        self.coordinates.as_str()
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct NewMasterTaskRequest {
    pub master_plan_id : String,
    pub abstract_task_id : String,
    pub duration : i32,
    pub min : i32,
    pub max : i32,
    pub task_type : String,
    pub coach_id : String,
    pub role_id : String,
    pub coordinates : String
}

impl NewMasterTaskRequest { 
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors: Vec<ValidationError> = Vec::new();

        if self.master_plan_id.trim().is_empty() {
            errors.push(ValidationError::new(
                "master_task_id",
                "Master Task Id is a must.",
            ));
        }

        if self.abstract_task_id.trim().is_empty() {
            errors.push(ValidationError::new(
                "abstract_task_id",
                "Abstract Task Id is a must"
            ));
        }

        if self.duration <= 0 {
            errors.push(ValidationError::new(
                "duration",
                "should be a minimum of 1 minute.",
            ));
        }

        if self.coach_id.trim().is_empty() {
            errors.push(ValidationError::new(
                "coach_id",
                "Coach Id is a must"
            ));
        }

        if self.role_id.trim().is_empty() {
            errors.push(ValidationError::new(
                "role_id",
                "Role Id is a must"
            ));
        }

        if self.task_type.trim().is_empty() {
            errors.push(ValidationError::new(
                "task_type",
                "Please provide Task Type"
            ));
        }

        errors

    }
}

#[derive(Insertable)]
#[table_name = "master_tasks"]
pub struct NewMasterTask {
    pub id : String,
    pub master_plan_id : String,
    pub abstract_task_id : String,
    pub duration : i32,
    pub task_type : String,
    pub coach_id : String,
    pub role_id : String,
    pub coordinates : String
}

impl NewMasterTask {
    pub fn from(request: &NewMasterTaskRequest) -> NewMasterTask {

        let fuzzy_id = util::fuzzy_id();


        let new_master_task = NewMasterTask {
            id: fuzzy_id,
            master_plan_id: request.master_plan_id.to_owned(),
            abstract_task_id: request.abstract_task_id.to_owned(),
            duration: request.duration,
            task_type: request.task_type.to_owned(),
            coach_id: request.coach_id.to_owned(),
            role_id: request.role_id.to_owned(),
            coordinates : request.coordinates.to_owned()
        };

        new_master_task
    }
}

#[derive(AsChangeset)]
#[table_name = "master_tasks"]
pub struct UpdateMasterTask {
    pub duration : i32,
    pub min : i32,
    pub max : i32,
    pub task_type : String,
    pub role_id : String,
    pub coordinates : String
}

#[derive(juniper::GraphQLInputObject)]
pub struct UpdateMasterTaskRequest {
    pub id : String,
    pub duration : i32,
    pub min : i32,
    pub max : i32,
    pub task_type : String,
    pub role_id : String,
    pub coordinates : String
}

impl UpdateMasterTaskRequest { 
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors: Vec<ValidationError> = Vec::new();

        if self.id.trim().is_empty(){
            errors.push(ValidationError::new("id","Id is a must."));
        }

        if self.role_id.trim().is_empty(){
            errors.push(ValidationError::new("role_id","Role Id is a must."));
        }

        if self.duration <= 0 {
            errors.push(ValidationError::new(
                "duration",
                "should be a minimum of 1 minute.",
            ));
        }

        errors
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct MasterTaskCriteria {
    pub master_plan_id: String,
}