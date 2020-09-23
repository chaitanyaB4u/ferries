use diesel::prelude::*;

use crate::schema::master_tasks::dsl::*;
use crate::models::master_tasks::{NewMasterTaskRequest, MasterTask, NewMasterTask, UpdateMasterTaskRequest, UpdateMasterTask};
use crate::models::master_tasks::{MasterTaskCriteria};

pub fn create_master_task(connection: &MysqlConnection, request: &NewMasterTaskRequest) -> Result<MasterTask, diesel::result::Error> {

    let new_master_task = NewMasterTask::from(request);

    diesel::insert_into(master_tasks).values(&new_master_task).execute(connection)?;

    master_tasks.filter(id.eq(new_master_task.id.to_owned())).first(connection)
}

pub fn update_master_task(connection: &MysqlConnection, request: &UpdateMasterTaskRequest) -> Result<MasterTask, diesel::result::Error> {
    let the_id = &request.id.as_str();


    diesel::update(master_tasks.filter(id.eq(the_id)))
    .set(&UpdateMasterTask{
        duration : request.duration,
        min : request.min,
        max : request.max,
        task_type : request.task_type.to_owned(),
        role_id : request.role_id.to_owned(),
        coordinates : request.coordinates.to_owned()
    })
    .execute(connection)?;

    master_tasks.filter(id.eq(the_id)).first(connection)
}

pub fn get_master_tasks(connection: &MysqlConnection, criteria: MasterTaskCriteria) -> Result<Vec<MasterTask>,diesel::result::Error> {

    master_tasks
        .filter(master_plan_id.eq(criteria.master_plan_id))
        .load(connection)
}