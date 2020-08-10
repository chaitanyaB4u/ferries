use diesel::prelude::*;

use crate::models::options::{NewOption,NewOptionRequest,Constraint};
use crate::schema::options::dsl::*;
use crate::models::enrollments::PlanCriteria;

pub fn create_option(connection: &MysqlConnection, request: &NewOptionRequest) -> Result<Constraint, diesel::result::Error> {

    let new_option = NewOption::from(request);

    diesel::insert_into(options).values(&new_option).execute(connection)?;
    
    options.filter(id.eq(new_option.id.to_owned())).first(connection)
}

pub fn get_options(connection: &MysqlConnection, criteria: PlanCriteria) -> Result<Vec<Constraint>,diesel::result::Error> {

    options
        .filter(enrollment_id.eq(criteria.enrollment_id))
        .load(connection)
}