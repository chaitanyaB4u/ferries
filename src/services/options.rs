use diesel::prelude::*;

use crate::models::enrollments::PlanCriteria;
use crate::models::options::{Constraint, NewOption, NewOptionRequest, UpdateOptionRequest};
use crate::schema::options::dsl::*;

pub fn create_option(connection: &MysqlConnection, request: &NewOptionRequest) -> Result<Constraint, diesel::result::Error> {
    let new_option = NewOption::from(request);

    diesel::insert_into(options).values(&new_option).execute(connection)?;

    options.filter(id.eq(new_option.id.to_owned())).first(connection)
}

pub fn update_option(connection: &MysqlConnection, request: &UpdateOptionRequest) -> Result<Constraint, diesel::result::Error> {
    let the_id = &request.id.as_str();

    diesel::update(options).filter(id.eq(the_id)).set(description.eq(request.description.to_owned())).execute(connection)?;

    options.filter(id.eq(the_id)).first(connection)
}

pub fn get_options(connection: &MysqlConnection, criteria: PlanCriteria) -> Result<Vec<Constraint>, diesel::result::Error> {
    options.filter(enrollment_id.eq(criteria.enrollment_id)).order_by(created_at.asc()).load(connection)
}
