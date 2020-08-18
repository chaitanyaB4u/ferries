use diesel::prelude::*;

use crate::commons::util;
use chrono::{Duration};

use crate::models::tasks::{NewTask,NewTaskRequest,Task, UpdateTaskRequest, UpdateTask};
use crate::schema::tasks::dsl::*;
use crate::models::enrollments::PlanCriteria;

pub fn create_task(connection: &MysqlConnection, request: &NewTaskRequest) -> Result<Task, diesel::result::Error> {

    let new_task = NewTask::from(request);

    diesel::insert_into(tasks).values(&new_task).execute(connection)?;

    tasks.filter(id.eq(new_task.id.to_owned())).first(connection)
}

pub fn update_task(connection: &MysqlConnection, request: &UpdateTaskRequest) -> Result<Task, diesel::result::Error> {
    let the_id = &request.id.as_str();
    
    let start_date = util::as_date(request.start_time.as_str());
    let given_duration = Duration::hours(request.duration as i64);
    let end_date = start_date.checked_add_signed(given_duration);

    diesel::update(tasks.filter(id.eq(the_id)))
    .set(&UpdateTask{
        description:request.description.to_owned(),
        name:request.name.to_owned(),
        duration:request.duration,
        original_start_date:start_date,
        original_end_date:end_date.unwrap_or(start_date)
    })
    .execute(connection)?;

    tasks.filter(id.eq(the_id)).first(connection)

}

pub fn get_tasks(connection: &MysqlConnection, criteria: PlanCriteria) -> Result<Vec<Task>,diesel::result::Error> {

    tasks
        .filter(enrollment_id.eq(criteria.enrollment_id))
        .order_by(original_start_date.asc())
        .load(connection)
}