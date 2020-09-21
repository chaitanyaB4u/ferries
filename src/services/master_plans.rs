use diesel::prelude::*;

use crate::models::master_plans::{NewMasterPlan,NewMasterPlanRequest,MasterPlan,MasterPlanCriteria};
use crate::schema::master_plans::dsl::*;

pub fn create_master_plan(connection: &MysqlConnection, request: &NewMasterPlanRequest) -> Result<MasterPlan, diesel::result::Error> {

    let new_master_plan = NewMasterPlan::from(request);

    diesel::insert_into(master_plans).values(&new_master_plan).execute(connection)?;

    master_plans.filter(id.eq(new_master_plan.id.to_owned())).first(connection)
}

pub fn get_master_plans(connection: &MysqlConnection, criteria: &MasterPlanCriteria) -> Result<Vec<MasterPlan>,diesel::result::Error> {
    master_plans
        .filter(coach_id.eq(&criteria.coach_id))
        .order_by(name.asc())
        .load(connection)
}