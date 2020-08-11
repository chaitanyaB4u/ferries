use crate::commons::chassis::ValidationError;
use crate::commons::util;
use crate::schema::tasks;

use chrono::{Duration, NaiveDateTime};

#[derive(Queryable, Debug, Identifiable)]
pub struct Task {
    pub id: String,
    pub enrollment_id: String,
    pub actor_id: String,
    pub name: String,
    pub duration: i32,
    pub min: i32,
    pub max: i32,
    pub original_start_date: NaiveDateTime,
    pub original_end_date: NaiveDateTime,
    pub revised_start_date: Option<NaiveDateTime>,
    pub revised_end_date: Option<NaiveDateTime>,
    pub offered_start_date: Option<NaiveDateTime>,
    pub offered_end_date: Option<NaiveDateTime>,
    pub actual_start_date: Option<NaiveDateTime>,
    pub actual_end_date: Option<NaiveDateTime>,
    pub locked: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub description: Option<String>,
}

#[derive(juniper::GraphQLEnum)]
enum Status {
    DONE,
    PLANNED,
    PROGRESS,
    DUE,
    DELAY,
}

#[juniper::object]
impl Task {
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn enrollment_id(&self) -> &str {
        self.enrollment_id.as_str()
    }

    pub fn actor_id(&self) -> &str {
        self.actor_id.as_str()
    }

    pub fn duration(&self) -> i32 {
        self.duration
    }

    pub fn min(&self) -> i32 {
        self.min
    }

    pub fn max(&self) -> i32 {
        self.max
    }

    pub fn scheduleStart(&self) -> NaiveDateTime {
        self.revised_start_date.unwrap_or(self.original_start_date)
    }

    pub fn scheduleEnd(&self) -> NaiveDateTime {
        self.revised_end_date.unwrap_or(self.original_end_date)
    }

    pub fn createdAt(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn status(&self) -> Status {
        if self.actual_end_date.is_some() {
            return Status::DONE;
        }
        if self.actual_start_date.is_some() {
            return Status::PROGRESS;
        }

        let rev_start_date = self.revised_start_date.unwrap_or(self.original_start_date);

        if util::is_past_date(rev_start_date) {
            return Status::DUE;
        }

        let rev_end_date = self.revised_end_date.unwrap_or(self.original_end_date);

        if util::is_past_date(rev_end_date) {
            return Status::DELAY;
        }

        Status::PLANNED
    }

    pub fn description(&self) -> &str {
        let value: &str = match &self.description {
            None=>"_",
            Some(value)=>value.as_str()
        };
        value
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct NewTaskRequest {
    pub enrollment_id: String,
    pub actor_id: String,
    pub start_time: String,
    pub duration: i32,
    pub description: String,
    pub name: String,
}

impl NewTaskRequest {
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors: Vec<ValidationError> = Vec::new();

        let given_time = self.start_time.as_str();

        if !util::is_valid_date(given_time) {
            errors.push(ValidationError::new("start_time", "unparsable date."));
        }

        let date = util::as_date(given_time);
        if util::is_past_date(date) {
            errors.push(ValidationError::new(
                "start_time",
                "should be a future date.",
            ));
        }

        if self.duration <= 0 {
            errors.push(ValidationError::new(
                "duration",
                "should be a minimum of 1 hour.",
            ));
        }

        if self.enrollment_id.trim().is_empty() {
            errors.push(ValidationError::new(
                "enrollment_id",
                "Enrollment Id is a must.",
            ));
        }

        errors
    }
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTask {
    pub id: String,
    pub enrollment_id: String,
    pub actor_id: String,
    pub duration: i32,
    pub original_start_date: NaiveDateTime,
    pub original_end_date: NaiveDateTime,
    pub description: String,
    pub name: String,
}

impl NewTask {
    pub fn from(request: &NewTaskRequest) -> NewTask {
        let start_date = util::as_date(request.start_time.as_str());
        let duration = Duration::hours(request.duration as i64);
        let end_date = start_date.checked_add_signed(duration);

        let fuzzy_id = util::fuzzy_id();

        let new_task = NewTask {
            id: fuzzy_id,
            enrollment_id: request.enrollment_id.to_owned(),
            actor_id: request.actor_id.to_owned(),
            duration: request.duration,
            original_start_date: start_date,
            original_end_date: end_date.unwrap_or(start_date),
            description: request.description.to_owned(),
            name:request.name.to_owned(),
        };

        new_task
    }
}
