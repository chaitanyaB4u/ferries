use crate::models::session_users::SessionUser;

use crate::schema::session_files;
use crate::schema::session_notes;

use crate::commons::chassis::ValidationError;
use crate::commons::util;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug)]
pub struct Note {
    pub id: String,
    pub session_id: String,
    pub created_by_id: String,
    pub session_user_id: String,
    pub description: String,
    pub remind_at: Option<NaiveDateTime>,
    pub is_private: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[juniper::object(description = "The fields we offer to the Web-UI ")]
impl Note {
    pub fn id(&self) -> &str {
        self.id.as_str()
    }
    pub fn session_id(&self) -> &str {
        self.session_id.as_str()
    }
    pub fn description(&self) -> &str {
        self.description.as_str()
    }
    pub fn is_private(&self) -> bool {
        self.is_private
    }
    pub fn remind_at(&self) -> Option<NaiveDateTime> {
        self.remind_at
    }
    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct NewNoteRequest {
    pub session_user_id: String,
    pub description: String,
    pub files: Option<Vec<FileRequest>>,
    pub remind_at: Option<String>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct FileRequest {
    pub path: String,
    pub name: String,
    pub r#type: String,
    pub size: i32,
}

impl NewNoteRequest {
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors: Vec<ValidationError> = Vec::new();

        if self.session_user_id.trim().is_empty() {
            errors.push(ValidationError::new("session_user_id", "Missing the session_user_fuzzy_id"));
        }

        if self.description.trim().is_empty() {
            errors.push(ValidationError::new("desciption", "Description of the note is a must."));
        }

        errors
    }
}

#[derive(Insertable)]
#[table_name = "session_notes"]
pub struct NewNote {
    pub id: String,
    pub session_id: String,
    pub created_by_id: String,
    pub session_user_id: String,
    pub description: String,
    pub remind_at: Option<NaiveDateTime>,
}

impl NewNote {
    pub fn from(request: &NewNoteRequest, session_user: SessionUser) -> NewNote {
        let remind_at = match &request.remind_at {
            Some(value) => {
                let date = util::as_date(value.as_str());
                Some(date)
            }
            None => None,
        };

        let fuzzy_id = util::fuzzy_id();

        NewNote {
            id: fuzzy_id,
            session_id: session_user.session_id,
            created_by_id: session_user.user_id,
            description: request.description.to_owned(),
            session_user_id: session_user.id,
            remind_at,
        }
    }
}

#[derive(Insertable)]
#[table_name = "session_files"]
pub struct NewNoteFile {
    pub id: String,
    pub session_note_id: String,
    pub file_name: String,
    pub file_path: String,
    pub file_type: Option<String>,
    pub file_size: Option<i32>,
}

impl NewNoteFile {
    pub fn from(request: &FileRequest, session_note_id: String) -> NewNoteFile {
        let fuzzy_id = util::fuzzy_id();

        NewNoteFile {
            id: fuzzy_id,
            session_note_id,
            file_path: request.path.to_owned(),
            file_name: request.name.to_owned(),
            file_type: Some(request.r#type.to_owned()),
            file_size: Some(request.size),
        }
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct NoteCriteria {
    pub session_user_id: String,
}