use diesel::prelude::*;

use crate::models::objectives::{NewObjective,NewObjectiveRequest,Objective};
use crate::schema::objectives::dsl::*;
use crate::models::enrollments::PlanCriteria;

pub fn create_objective(connection: &MysqlConnection, request: &NewObjectiveRequest) -> Result<Objective, diesel::result::Error> {

    let new_objective = NewObjective::from(request);
    
    diesel::insert_into(objectives).values(&new_objective).execute(connection)?;
    
    objectives.filter(id.eq(new_objective.id.to_owned())).first(connection)
}

/**
 * Let us stuff the content form the file system
 */
pub fn get_objectives(connection: &MysqlConnection, criteria: PlanCriteria) -> Result<Vec<Objective>,diesel::result::Error> {

    objectives
        .filter(enrollment_id.eq(criteria.enrollment_id))
        .load(connection)
}