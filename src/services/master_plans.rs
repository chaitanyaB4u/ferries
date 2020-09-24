use diesel::prelude::*;

use crate::commons::util;
use crate::models::master_plans::UpdateMasterPlanRequest;
use crate::models::master_plans::{MasterPlan, MasterPlanCriteria, NewMasterPlan, NewMasterPlanRequest, TaskUnit};

use crate::schema::master_plans;
use crate::schema::master_task_links;
use crate::schema::master_tasks;

use crate::schema::master_plans::dsl::*;
use crate::schema::master_task_links::dsl::*;
use crate::schema::master_tasks::dsl::*;

pub fn create_master_plan(connection: &MysqlConnection, request: &NewMasterPlanRequest) -> Result<MasterPlan, diesel::result::Error> {
    let new_master_plan = NewMasterPlan::from(request);

    diesel::insert_into(master_plans).values(&new_master_plan).execute(connection)?;

    master_plans.filter(master_plans::id.eq(new_master_plan.id.to_owned())).first(connection)
}

pub fn get_master_plans(connection: &MysqlConnection, criteria: &MasterPlanCriteria) -> Result<Vec<MasterPlan>, diesel::result::Error> {
    master_plans.filter(master_plans::coach_id.eq(&criteria.coach_id)).order_by(name.asc()).load(connection)
}

fn delete_current_links(connection: &MysqlConnection, plan_id: &str) -> Result<String, diesel::result::Error> {
    let current_links = master_task_links.filter(master_task_links::master_plan_id.eq(plan_id));
    diesel::delete(current_links).execute(connection)?;

    Ok(String::from("Ok"))
}

fn deletable_tasks(connection: &MysqlConnection, plan_id: &str, given_tasks: &Vec<TaskUnit>) -> Result<Vec<String>, diesel::result::Error> {
    let current_task_ids: Vec<String> = master_tasks.filter(master_tasks::master_plan_id.eq(plan_id)).select(master_tasks::id).load(connection)?;
    let mut given_ids: Vec<String> = given_tasks.iter().map(|tu| tu.id.clone()).collect();
    given_ids.sort_unstable_by(|id1, id2| id1.cmp(&id2));

    let tasks_to_delete: Vec<String> = util::find_diff(current_task_ids, given_ids);

    Ok(tasks_to_delete)
}

pub fn update_master_plan(connection: &MysqlConnection, request: &UpdateMasterPlanRequest) -> Result<String, diesel::result::Error> {
    let plan_id: String = request.master_plan_id.to_owned();

    delete_current_links(connection, request.master_plan_id.as_str())?;

    let tasks_to_delete = deletable_tasks(connection, &plan_id, &request.tasks)?;

    for task_id in tasks_to_delete {
        diesel::delete(master_tasks.filter(master_tasks::id.eq(task_id))).execute(connection)?;
    }

    for task in &request.tasks {
        let a_task = master_tasks.filter(master_tasks::id.eq(&task.id));
        diesel::update(a_task).set(master_tasks::coordinates.eq(&task.coordinates)).execute(connection)?;
    }

    let new_links = request.as_master_task_links();
    diesel::insert_into(master_task_links).values(&new_links).execute(connection)?;

    Ok(String::from("Ok"))
}
