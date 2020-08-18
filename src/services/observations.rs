use diesel::prelude::*;

use crate::models::observations::{NewObservation,NewObservationRequest,Observation,UpdateObservationRequest};
use crate::schema::observations::dsl::*;
use crate::models::enrollments::PlanCriteria;

pub fn create_observation(connection: &MysqlConnection, request: &NewObservationRequest) -> Result<Observation, diesel::result::Error> {

    let new_observation = NewObservation::from(request);

    diesel::insert_into(observations).values(&new_observation).execute(connection)?;
    
    observations.filter(id.eq(new_observation.id.to_owned())).first(connection)
}

pub fn update_observation(connection: &MysqlConnection, request: &UpdateObservationRequest) -> Result<Observation, diesel::result::Error> {
    
    let the_id = &request.id.as_str();
 
    diesel::update(observations)
    .filter(id.eq(the_id))
    .set(description.eq(request.description.to_owned()))
    .execute(connection)?;

    observations.filter(id.eq(the_id)).first(connection)
}

pub fn get_observations(connection: &MysqlConnection, criteria: PlanCriteria) -> Result<Vec<Observation>,diesel::result::Error> {

    observations
        .filter(enrollment_id.eq(criteria.enrollment_id))
        .order_by(created_at.asc())
        .load(connection)
}