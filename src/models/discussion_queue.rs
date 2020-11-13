use crate::schema::discussion_queue;

use crate::models::discussions::NewDiscussionRequest;

use crate::models::users::User;

use chrono::NaiveDateTime;

#[derive(Queryable, Debug)]
pub struct Feed {
    pub id: String,
    pub to_id: String,
    pub discussion_id: String,
    pub created_at: NaiveDateTime,
    pub is_pending: bool,
    pub enrollment_id: String,
    pub program_id: String,
    pub program_name: String,
    pub coach_id: String,
    pub coach_name: String,
    pub member_id: String,
    pub member_name: String,
}

#[juniper::object]
impl Feed {
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn to_id(&self) -> &str {
        self.to_id.as_str()
    }

    pub fn enrollment_id(&self) -> &str {
        self.enrollment_id.as_str()
    }

    pub fn program_id(&self) -> &str {
        self.program_id.as_str()
    }

    pub fn program_name(&self) -> &str {
        self.program_name.as_str()
    }

    pub fn coach_id(&self) -> &str {
        self.coach_id.as_str()
    }

    pub fn coach_name(&self) -> &str {
        self.coach_name.as_str()
    }

    pub fn member_id(&self) -> &str {
        self.member_id.as_str()
    }

    pub fn member_name(&self) -> &str {
        self.member_name.as_str()
    }

}

#[derive(Insertable)]
#[table_name = "discussion_queue"]
pub struct NewFeed {
    pub id: String,
    pub to_id: String,
    pub discussion_id: String,
    pub enrollment_id: String,
    pub program_id: String,
    pub program_name: String,
    pub coach_id: String,
    pub coach_name: String,
    pub member_id: String,
    pub member_name: String   
}

impl NewFeed {
    pub fn from(request: &NewDiscussionRequest, discussion_id: &str) -> NewFeed {
        NewFeed {
            id: discussion_id.to_owned(),
            discussion_id: discussion_id.to_owned(),
            to_id: request.to_id.to_owned(),
            enrollment_id: request.enrollment_id.to_owned(),
            program_id: request.program_id.to_owned(),
            program_name: request.program_name.to_owned(),
            coach_id: request.coach_id.to_owned(),
            coach_name: request.coach_name.to_owned(),
            member_id: request.member_id.to_owned(),
            member_name: request.member_name.to_owned(),
        }
    }
}

pub struct PendingFeed {
    pub description: String,
    pub feed: Feed,
    pub user: User,
}

#[juniper::object]
impl PendingFeed {

    pub fn description(&self) -> &str {
        &self.description.as_str()
    }

    pub fn feed(&self) -> &Feed {
        &self.feed
    }

    pub fn user(&self) -> &User {
        &self.user
    }
}
