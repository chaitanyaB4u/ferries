use diesel::prelude::*;

use crate::models::observations::{NewObservation,NewObservationRequest,Observation};
use crate::schema::observations::dsl::*;
use crate::models::enrollments::PlanCriteria;

pub fn create_observation(connection: &MysqlConnection, request: &NewObservationRequest) -> Result<Observation, diesel::result::Error> {

    let new_observation = NewObservation::from(request);

    diesel::insert_into(observations).values(&new_observation).execute(connection)?;
    
    observations.filter(id.eq(new_observation.id.to_owned())).first(connection)
}

pub fn get_observations(connection: &MysqlConnection, criteria: PlanCriteria) -> Result<Vec<Observation>,diesel::result::Error> {

    observations
        .filter(enrollment_id.eq(criteria.enrollment_id))
        .load(connection)
}