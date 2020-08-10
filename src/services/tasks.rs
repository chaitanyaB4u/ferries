use diesel::prelude::*;

use crate::models::tasks::{NewTask,NewTaskRequest,Task};
use crate::schema::tasks::dsl::*;

pub fn create_task(connection: &MysqlConnection, request: &NewTaskRequest) -> Result<Task, diesel::result::Error> {

    let new_task = NewTask::from(request);

    diesel::insert_into(tasks).values(&new_task).execute(connection)?;

    tasks.filter(id.eq(new_task.id.to_owned())).first(connection)
}