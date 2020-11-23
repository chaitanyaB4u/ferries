use crate::commons::util;
use diesel::prelude::*;

use crate::models::enrollments::PlanCriteria;
use crate::models::objectives::{NewObjective, NewObjectiveRequest, Objective, UpdateObjective, UpdateObjectiveRequest};
use crate::schema::objectives::dsl::*;

pub fn create_objective(connection: &MysqlConnection, request: &NewObjectiveRequest) -> Result<Objective, diesel::result::Error> {
    let new_objective = NewObjective::from(request);

    diesel::insert_into(objectives).values(&new_objective).execute(connection)?;

    objectives.filter(id.eq(new_objective.id)).first(connection)
}

pub fn update_objective(connection: &MysqlConnection, request: &UpdateObjectiveRequest) -> Result<Objective, diesel::result::Error> {
    let the_id = &request.id.as_str();

    let start_date = util::as_date(request.start_time.as_str());
    let end_date = util::as_date(request.end_time.as_str());

    diesel::update(objectives.filter(id.eq(the_id)))
        .set(&UpdateObjective {
            description: request.description.to_owned(),
            original_start_date: start_date,
            original_end_date: end_date,
        })
        .execute(connection)?;

    objectives.filter(id.eq(the_id)).first(connection)
}
/**
 * Let us stuff the content form the file system
 */
pub fn get_objectives(connection: &MysqlConnection, criteria: PlanCriteria) -> Result<Vec<Objective>, diesel::result::Error> {
    objectives.filter(enrollment_id.eq(criteria.enrollment_id)).order_by(original_start_date.asc()).load(connection)
}
