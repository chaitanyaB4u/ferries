use diesel::prelude::*;

use crate::models::tasks::{NewTask,NewTaskRequest,Task};
use crate::schema::tasks::dsl::*;
use crate::models::enrollments::PlanCriteria;

pub fn create_task(connection: &MysqlConnection, request: &NewTaskRequest) -> Result<Task, diesel::result::Error> {

    let new_task = NewTask::from(request);

    diesel::insert_into(tasks).values(&new_task).execute(connection)?;

    tasks.filter(id.eq(new_task.id.to_owned())).first(connection)
}

pub fn get_tasks(connection: &MysqlConnection, criteria: PlanCriteria) -> Result<Vec<Task>,diesel::result::Error> {

    tasks
        .filter(enrollment_id.eq(criteria.enrollment_id))
        .load(connection)
}