use crate::commons::chassis::ValidationError;
use crate::commons::util;
use crate::schema::master_plans;
use crate::schema::master_task_links;

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

        NewMasterPlan {
            id: fuzzy_id,
            name: request.name.to_owned(),
            coach_id: request.coach_id.to_owned(),
            description: request.description.to_owned(),
        }
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct MasterPlanCriteria {
    pub coach_id: String,
}

#[derive(juniper::GraphQLInputObject)]
pub struct UpdateMasterPlanRequest {
    pub master_plan_id: String,
    pub tasks: Vec<TaskUnit>,
    pub links: Vec<LinkUnit>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct TaskUnit {
    pub id: String,
    pub coordinates: String,
}

#[derive(juniper::GraphQLInputObject)]
pub struct LinkUnit {
    pub source_id: String,
    pub target_id: String,
    pub coordinates: String,
    pub priority: i32,
    pub is_forward: bool,
}

impl UpdateMasterPlanRequest {
    pub fn as_master_task_links(&self) -> Vec<NewMasterTaskLink> {
        let mut new_links = Vec::new();

        for link in &self.links {
            let new_link = NewMasterTaskLink::from(link, self.master_plan_id.as_str());
            new_links.push(new_link);
        }

        new_links
    }
}

#[derive(Insertable)]
#[table_name = "master_task_links"]
pub struct NewMasterTaskLink {
    pub id: String,
    pub master_plan_id: String,
    pub source_task_id: String,
    pub target_task_id: String,
    pub coordinates: String,
    pub priority: i32,
    pub is_forward: bool,
}

impl NewMasterTaskLink {
    pub fn from(link: &LinkUnit, plan_id: &str) -> NewMasterTaskLink {
        let fuzzy_id = util::fuzzy_id();

        NewMasterTaskLink {
            id: fuzzy_id,
            master_plan_id: plan_id.to_owned(),
            source_task_id: link.source_id.to_owned(),
            target_task_id: link.target_id.to_owned(),
            coordinates: link.coordinates.to_owned(),
            priority: link.priority,
            is_forward: link.is_forward,
        }
    }
}
