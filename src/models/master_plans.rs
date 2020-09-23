use crate::commons::chassis::ValidationError;
use crate::commons::util;
use crate::schema::master_plans;


#[derive(Queryable, Debug, Identifiable)]
pub struct MasterPlan {
    pub id: String,
    pub name: String,
    pub description: String,
    pub coach_id: String,
}

#[juniper::object]
impl MasterPlan {
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn coach_id(&self) -> &str {
        self.coach_id.as_str()
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct NewMasterPlanRequest {
    pub name: String,
    pub description: String,
    pub coach_id: String,
}

impl NewMasterPlanRequest {
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors: Vec<ValidationError> = Vec::new();

        if self.name.trim().is_empty() {
            errors.push(ValidationError::new("name", "Name is a must."));
        }

        if self.description.trim().is_empty() {
            errors.push(ValidationError::new("description", "Description is a must."));
        }

        if self.coach_id.trim().is_empty() {
            errors.push(ValidationError::new("coach_id", "Coach is a must."));
        }

        errors
    }
}

#[derive(Insertable)]
#[table_name = "master_plans"]
pub struct NewMasterPlan {
    pub id: String,
    pub name: String,
    pub description: String,
    pub coach_id: String,
}

impl NewMasterPlan {
    pub fn from(request: &NewMasterPlanRequest) -> NewMasterPlan {
        let fuzzy_id = util::fuzzy_id();

        let new_master_plan = NewMasterPlan {
            id: fuzzy_id,
            name:request.name.to_owned(),
            coach_id: request.coach_id.to_owned(),
            description: request.description.to_owned()
        };

        new_master_plan
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct MasterPlanCriteria {
    pub coach_id: String,
}