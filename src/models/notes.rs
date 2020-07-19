
use crate::models::session_users::SessionUser;

use crate::schema::session_notes;
use crate::schema::session_files;

use crate::commons::chassis::{ValidationError};
use chrono::{NaiveDateTime};
use crate::commons::util;

#[derive(Queryable,Debug)]
pub struct Note {
    pub id: i32,
    pub fuzzy_id:  String,
    pub session_id:  i32,
    pub description: String,
    pub remind_at: Option<NaiveDateTime>,
    pub created_by_id: i32,
    pub is_private: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub session_user_id: i32,
}

#[juniper::object(description="The fields we offer to the Web-UI ")]
impl Note {

    pub fn fuzzy_id(&self) -> &str {
        self.fuzzy_id.as_str()
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
pub struct NewNoteRequest{
    pub session_user_fuzzy_id:  String,
    pub description: String,
    pub files: Option<Vec<FileRequest>>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct FileRequest {
    pub path: String,
    pub name: String,
    pub r#type: String,
    pub size: i32,
}


impl NewNoteRequest {

    pub fn validate(&self) ->Vec<ValidationError> {

        let mut errors: Vec<ValidationError> = Vec::new();

        if self.session_user_fuzzy_id.trim().is_empty() {
            errors.push(ValidationError::new("session_user_fuzzy_id", "Missing the session_user_fuzzy_id"));
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
    pub session_id:  i32,
    pub created_by_id: i32,
    pub description: String,
    pub fuzzy_id: String,
    pub session_user_id: i32,
}

impl NewNote {

    pub fn from(request: &NewNoteRequest,session_user: SessionUser) -> NewNote {

        let fuzzy_id = util::fuzzy_id();

        NewNote {
            session_id:session_user.session_id,
            created_by_id:session_user.user_id,
            fuzzy_id:fuzzy_id,
            description:request.description.to_owned(),
            session_user_id:session_user.id
        }
    }
}


#[derive(Insertable)]
#[table_name = "session_files"]
pub struct NewNoteFile {
    pub fuzzy_id: String,
    pub session_note_id: i32,
    pub file_name: String,
    pub file_path: String,
    pub file_type: Option<String>,
    pub file_size: Option<i32>,
}


impl NewNoteFile {

    pub fn from(request: &FileRequest, session_note_id: i32) -> NewNoteFile {

        let fuzzy_id = util::fuzzy_id();

        NewNoteFile {
            session_note_id:session_note_id,
            fuzzy_id:fuzzy_id,
            file_path:request.path.to_owned(),
            file_name:request.name.to_owned(),
            file_type:Some(request.r#type.to_owned()),
            file_size:Some(request.size),
        }

    }
}
