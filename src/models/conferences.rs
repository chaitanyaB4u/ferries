use crate::commons::chassis::ValidationError;
use crate::commons::util;
use crate::schema::conferences;

use chrono::{Duration, NaiveDateTime};

#[derive(Queryable,Identifiable,Debug)]
pub struct Conference {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub program_id: String,
    pub people: Option<String>,
    pub duration: i32,
    pub original_start_date: NaiveDateTime,
    pub original_end_date: NaiveDateTime,
    pub revised_start_date: Option<NaiveDateTime>,
    pub revised_end_date: Option<NaiveDateTime>,
    pub offered_start_date: Option<NaiveDateTime>,
    pub offered_end_date: Option<NaiveDateTime>,
    pub is_ready: bool,
    pub actual_start_date: Option<NaiveDateTime>,
    pub actual_end_date: Option<NaiveDateTime>,
    pub cancelled_at: Option<NaiveDateTime>,
    pub closing_notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Conference  {
    
    pub fn description(&self) -> &str {
        let value: &str = match &self.description {
            None => "_",
            Some(value) => value.as_str(),
        };

        value
    }
}

#[juniper::object]
impl Conference {
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> &str {
        let value: &str = match &self.description {
            None => "_",
            Some(value) => value.as_str(),
        };

        value
    }

    pub fn program_id(&self) -> &str {
        self.program_id.as_str()
    }

    pub fn people(&self) -> &str {
        let value: &str = match &self.people {
            None => "_",
            Some(value) => value.as_str(),
        };

        value
    }

    pub fn duration(&self) -> i32 {
        self.duration
    }

    pub fn scheduleStart(&self) -> NaiveDateTime {
        self.revised_start_date.unwrap_or(self.original_start_date)
    }

    pub fn scheduleEnd(&self) -> NaiveDateTime {
        self.revised_end_date.unwrap_or(self.original_end_date)
    }

    pub fn actualStart(&self) -> Option<NaiveDateTime> {
        self.actual_start_date
    }

    pub fn actualEnd(&self) -> Option<NaiveDateTime> {
        self.actual_end_date
    }

    pub fn closing_notes(&self) -> Option<String> {
        self.closing_notes.clone()
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct NewConferenceRequest {
    pub program_id: String,
    pub name: String,
    pub description: String,
    pub duration: i32,
    pub start_time: String,
}

impl NewConferenceRequest {
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors: Vec<ValidationError> = Vec::new();

        let given_time = self.start_time.as_str();

        if !util::is_valid_date(given_time) {
            errors.push(ValidationError::new("start_time", "unparsable date."));
        }

        let date = util::as_date(given_time);
        if util::is_past_date(date) {
            errors.push(ValidationError::new("start_time", "should be a future date."));
        }

        if self.duration < 15 {
            errors.push(ValidationError::new("duration", "should be a minimum of 15 minutes"));
        }

        if self.program_id.trim().is_empty() {
            errors.push(ValidationError::new("program_id", "Program fuzzy id is a must."));
        }

        if self.name.trim().is_empty() {
            errors.push(ValidationError::new("name", "name of the conference is a must."));
        }

        if self.description.trim().is_empty() {
            errors.push(ValidationError::new("description", "description of the conference is a must."));
        }

        errors
    }
}

#[derive(Insertable)]
#[table_name = "conferences"]
pub struct NewConference {
    pub id: String,
    pub name: String,
    pub description: String,
    pub program_id: String,
    pub people: String,
    pub duration: i32,
    pub original_start_date: NaiveDateTime,
    pub original_end_date: NaiveDateTime,
}

impl NewConference {
    pub fn from(request: &NewConferenceRequest, people: String) -> NewConference {
        let start_date = util::as_date(request.start_time.as_str());
        let duration = Duration::minutes(request.duration as i64);
        let end_date = start_date.checked_add_signed(duration);

        let fuzzy_id = util::fuzzy_id();

        NewConference {
            id: fuzzy_id,
            name: request.name.to_owned(),
            description: request.description.to_owned(),
            program_id: request.program_id.to_owned(),
            people,
            duration: request.duration,
            original_start_date: start_date,
            original_end_date: end_date.unwrap_or(start_date),
        }
    }
}

#[derive(juniper::GraphQLEnum, PartialEq)]
pub enum IntentionState {
    ADD,
    REMOVE,
}

#[derive(juniper::GraphQLInputObject)]
pub struct MemberRequest {
    pub conference_id: String,
    pub member_ids: Vec<String>,
    pub intention: IntentionState,
}





