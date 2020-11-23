use diesel::prelude::*;

use crate::models::abstract_tasks::{AbstractTask, AbstractTaskCriteria, NewAbstractTask, NewAbstractTaskRequest};
use crate::schema::abstract_tasks::dsl::*;

pub fn create_abstract_task(connection: &MysqlConnection, request: &NewAbstractTaskRequest) -> Result<AbstractTask, diesel::result::Error> {
    let new_abstract_task = NewAbstractTask::from(request);

    diesel::insert_into(abstract_tasks).values(&new_abstract_task).execute(connection)?;

    abstract_tasks.filter(id.eq(new_abstract_task.id)).first(connection)
}

pub fn get_abstract_tasks(connection: &MysqlConnection, criteria: &AbstractTaskCriteria) -> Result<Vec<AbstractTask>, diesel::result::Error> {
    abstract_tasks.filter(coach_id.eq(&criteria.coach_id)).order_by(name.asc()).load(connection)
}
