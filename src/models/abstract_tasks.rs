use crate::commons::chassis::ValidationError;
use crate::commons::util;
use crate::schema::abstract_tasks;

#[derive(Queryable, Debug, Identifiable)]
pub struct AbstractTask {
    pub id: String,
    pub name: String,
    pub coach_id: String,
}

#[juniper::object]
impl AbstractTask {
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn coach_id(&self) -> &str {
        self.coach_id.as_str()
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct NewAbstractTaskRequest {
    pub name: String,
    pub coach_id: String,
}

impl NewAbstractTaskRequest {
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors: Vec<ValidationError> = Vec::new();

        if self.name.trim().is_empty() {
            errors.push(ValidationError::new("name", "Name is a must."));
        }

        if self.coach_id.trim().is_empty() {
            errors.push(ValidationError::new("coach_id", "Coach is a must."));
        }

        errors
    }
}

#[derive(Insertable)]
#[table_name = "abstract_tasks"]
pub struct NewAbstractTask {
    pub id: String,
    pub name: String,
    pub coach_id: String,
}

impl NewAbstractTask {
    pub fn from(request: &NewAbstractTaskRequest) -> NewAbstractTask {
        let fuzzy_id = util::fuzzy_id();

        NewAbstractTask {
            id: fuzzy_id,
            name: request.name.to_owned(),
            coach_id: request.coach_id.to_owned(),
        }
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct AbstractTaskCriteria {
    pub coach_id: String,
}
