use diesel::prelude::*;

use crate::commons::util;

use crate::file_manager::{get_file_names, SESSION_ASSET_DIR};

use crate::models::enrollments::{Enrollment, PlanCriteria};
use crate::models::notes::Note;
use crate::models::sessions::Session;
use crate::models::user_events::EventCriteria;

use crate::schema::enrollments;
use crate::schema::session_notes;
use crate::schema::sessions;

use crate::schema::enrollments::dsl::*;
use crate::schema::session_notes::dsl::*;
use crate::schema::sessions::dsl::*;

use std::path::PathBuf;

pub struct NoteRow {
    pub session: Session,
    pub note: Note,
    pub by: String,
}

#[juniper::object]
impl NoteRow {
    pub fn session(&self) -> &Session {
        &self.session
    }
    pub fn note(&self) -> &Note {
        &self.note
    }
    pub fn by(&self) -> &String {
        &self.by
    }
}

pub fn get_enrollment_notes(connection: &MysqlConnection, criteria: PlanCriteria) -> Result<Vec<NoteRow>, diesel::result::Error> {
    type Row = (Enrollment, (Session, Note));

    let artifact_rows: Vec<Row> = enrollments
        .inner_join(sessions.inner_join(session_notes))
        .filter(enrollments::id.eq(&criteria.enrollment_id))
        .order_by(session_notes::updated_at.asc())
        .load(connection)?;

    let mut rows: Vec<NoteRow> = Vec::new();
    for item in artifact_rows {
        let enrollment = item.0;
        let session = (item.1).0;
        let note = (item.1).1;

        let by = if enrollment.member_id == note.created_by_id { util::MEMBER } else { util::COACH };

        let note_row = NoteRow { session, note, by: String::from(by) };

        rows.push(note_row);
    }

    Ok(rows)
}

pub struct BoardRow {
    pub session: Session,
    pub urls: Vec<String>,
}

#[juniper::object]
impl BoardRow {
    pub fn session(&self) -> &Session {
        &self.session
    }

    pub fn urls(&self) -> &Vec<String> {
        &self.urls
    }
}

type Row = (Enrollment, Session);

pub fn get_boards(connection: &MysqlConnection, criteria: EventCriteria) -> Result<Vec<BoardRow>, diesel::result::Error> {
    let prog_id = criteria.program_id.unwrap();

    let rows: Vec<Row> = enrollments
        .inner_join(sessions)
        .filter(enrollments::program_id.eq(&prog_id))
        .filter(enrollments::member_id.eq(&criteria.user_id))
        .order_by(sessions::updated_at.asc())
        .load(connection)?;

    Ok(get_session_boards(&rows))
}

/**
 * Sessions without any boards will not be returned.
 * 
 * We store the boards against the conference id if the session is part
 * of a conference, So the url should be constructed with the conference id 
 * instead of session id for conference sessions.
 */
fn get_session_boards(rows: &[Row]) -> Vec<BoardRow> {
    let mut board_rows: Vec<BoardRow> = Vec::new();

    for row in rows {
        let mut dir_name: PathBuf = PathBuf::from(SESSION_ASSET_DIR);

        let artifact_id = match &row.1.conference_id {
            Some(value) => value.to_owned(),
            None => row.1.id.to_owned()
        };
        
        dir_name.push(artifact_id);
        dir_name.push("boards");

        if let Ok(urls) = get_file_names(dir_name) {
            board_rows.push(BoardRow {
                session: row.1.clone(),
                urls,
            });
        }
    }

    board_rows
}
