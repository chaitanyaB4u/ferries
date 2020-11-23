use diesel::dsl::count;
use diesel::prelude::*;

use crate::schema::discussion_queue;
use crate::schema::discussions;

use crate::schema::discussion_queue::dsl::*;
use crate::schema::discussions::dsl::*;
use crate::schema::users::dsl::*;

use crate::models::discussion_queue::{Feed, NewFeed, PendingFeed};
use crate::models::discussions::{Discussion, DiscussionCriteria, NewDiscussion, NewDiscussionRequest};
use crate::models::users::User;

use crate::models::users::UserCriteria;

const FEED_COUNT_ERROR: &str = "Error while counting pending feeds.";

pub fn create_new_discussion(connection: &MysqlConnection, request: &NewDiscussionRequest) -> QueryResult<Discussion> {
    let new_discussion = NewDiscussion::from(request);

    diesel::insert_into(discussions).values(&new_discussion).execute(connection)?;

    let discussion: Discussion = discussions.filter(discussions::id.eq(&new_discussion.id)).first(connection)?;

    let new_feed = NewFeed::from(&request, discussion.id.as_str());

    diesel::insert_into(discussion_queue).values(&new_feed).execute(connection)?;

    // Mark any prior pending feeds for the user as read
    mark_as_read(connection, request.created_by_id.as_str(), request.enrollment_id.as_str());

    Ok(discussion)
}

pub fn get_discussions(connection: &MysqlConnection, criteria: DiscussionCriteria) -> Result<Vec<Discussion>, diesel::result::Error> {
    discussions
        .filter(discussions::enrollment_id.eq(criteria.enrollment_id))
        .order_by(discussions::created_at.asc())
        .load(connection)
}

/**
*  Return the top 50 messages awaiting the user reponse in the  
*  descending order of the time stamp.
*  
*  We need to know the User who created the message.
*/

pub fn get_pending_discussions(connection: &MysqlConnection, criteria: &UserCriteria) -> Result<Vec<PendingFeed>, diesel::result::Error> {
    type FeedRow = (Feed, (Discussion,User));

    let rows: Vec<FeedRow> = discussion_queue
        .inner_join(discussions.inner_join(users))
        .filter(is_pending.eq(true))
        .filter(to_id.eq(criteria.id.as_str()))
        .order_by(discussions::created_at.desc())
        .limit(50)
        .load(connection)?;

    let result: Vec<PendingFeed> = rows.into_iter()
        .map(|tuple| PendingFeed { 
            feed: tuple.0, 
            description: ((tuple.1).0).description,
            user: (tuple.1).1 
            }
        )
        .collect();
    
    Ok(result)
}

pub fn get_pending_feed_count(connection: &MysqlConnection, user_id: &str) -> Result<i64, &'static str> {
    let result = discussion_queue
        .filter(is_pending.eq(true))
        .filter(to_id.eq(user_id))
        .select(count(is_pending))
        .first(connection);

    if result.is_err() {
        return Err(FEED_COUNT_ERROR);
    }

    Ok(result.unwrap())
}

/**
 * When a user respond or typed a message, it is understood that the user read all
 * those prior feeds, hence he will be marked as read.
 *
*/

fn mark_as_read(connection: &MysqlConnection, to_user_id: &str, for_enrollment_id: &str) {
    let target_feeds = discussion_queue
        .filter(is_pending.eq(true))
        .filter(to_id.eq(to_user_id))
        .filter(discussion_queue::enrollment_id.eq(for_enrollment_id));

    let _ = diesel::update(target_feeds).set(is_pending.eq(false)).execute(connection);
}
