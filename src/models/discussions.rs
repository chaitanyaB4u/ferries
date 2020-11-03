use crate::schema::discussions;

use crate::commons::util;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug)]
pub struct Discussion {
    pub id: String,
    pub enrollment_id: String,
    pub created_by_id: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[juniper::object]
impl Discussion {

    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn enrollment_id(&self) -> &str {
        self.enrollment_id.as_str()
    }

    pub fn created_by_id(&self) -> &str {
        self.created_by_id.as_str()
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }   
}

#[derive(juniper::GraphQLInputObject)]
pub struct NewDiscussionRequest {
    pub enrollment_id: String,
    pub created_by_id: String,
    pub description: String, 
}


#[derive(Insertable)]
#[table_name = "discussions"]
pub struct NewDiscussion {
    pub id: String,
    pub enrollment_id: String,
    pub created_by_id: String,
    pub description: String,
}

impl NewDiscussion {
    pub fn from(request: &NewDiscussionRequest) -> NewDiscussion{
        
        let fuzzy_id = util::fuzzy_id();

        NewDiscussion {
            id: fuzzy_id,
            enrollment_id: request.enrollment_id.to_owned(),
            created_by_id: request.created_by_id.to_owned(),
            description: request.description.to_owned()
        }
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct DiscussionCriteria {
    pub enrollment_id: String
}