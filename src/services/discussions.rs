use diesel::prelude::*;

use crate::schema::discussions::dsl::*;
use crate::models::discussions::{NewDiscussionRequest,Discussion,NewDiscussion, DiscussionCriteria};

pub fn create_new_discussion(connection: &MysqlConnection, request: &NewDiscussionRequest) -> QueryResult<Discussion> {

    let new_discussion = NewDiscussion::from(request);

    diesel::insert_into(discussions).values(&new_discussion).execute(connection)?;

    discussions.filter(id.eq(&new_discussion.id)).first(connection)

}

pub fn get_discussions(connection: &MysqlConnection, criteria: DiscussionCriteria) -> Result<Vec<Discussion>, diesel::result::Error> {
    discussions
    .filter(enrollment_id.eq(criteria.enrollment_id))
    .order_by(created_at.asc())
    .load(connection)
}