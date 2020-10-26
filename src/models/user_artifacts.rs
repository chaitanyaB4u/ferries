use diesel::prelude::*;

use crate::commons::util;

use crate::file_manager::{get_file_names, SESSION_ASSET_DIR};

use crate::models::enrollments::{Enrollment, PlanCriteria};
use crate::models::notes::Note;
use crate::models::session_users::SessionUser;
use crate::models::sessions::Session;
use crate::models::user_events::EventCriteria;

use crate::schema::enrollments;
use crate::schema::session_notes;
use crate::schema::sessions;

use crate::schema::enrollments::dsl::*;
use crate::schema::session_notes::dsl::*;
use crate::schema::session_users::dsl::*;
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

        let mut by = util::COACH;
        if enrollment.member_id == note.created_by_id {
            by = util::MEMBER;
        }

        let note_row = NoteRow { session, note, by: String::from(by) };

        rows.push(note_row);
    }

    Ok(rows)
}

pub struct BoardRow {
    pub session: Session,
    pub session_user: SessionUser,
    pub urls: Vec<String>,
}

#[juniper::object]
impl BoardRow {
    pub fn session(&self) -> &Session {
        &self.session
    }

    pub fn session_user(&self) -> &SessionUser {
        &self.session_user
    }

    pub fn urls(&self) -> &Vec<String> {
        &self.urls
    }
}

type Row = (Enrollment, (Session, SessionUser));

pub fn get_boards(connection: &MysqlConnection, criteria: EventCriteria) -> Result<Vec<BoardRow>, diesel::result::Error> {
    let prog_id = criteria.program_id.unwrap();

    let rows: Vec<Row> = enrollments
        .inner_join(sessions.inner_join(session_users))
        .filter(enrollments::program_id.eq(&prog_id))
        .filter(enrollments::member_id.eq(&criteria.user_id))
        .order_by(sessions::updated_at.asc())
        .load(connection)?;

    Ok(get_session_boards(&rows))

}

/**
 * Sessions without any boards will not be returned.
 */
fn get_session_boards(rows: &Vec<Row>) -> Vec<BoardRow> {
    let mut board_rows: Vec<BoardRow> = Vec::new();

    for row in rows {
        let mut dir_name: PathBuf = PathBuf::from(SESSION_ASSET_DIR);
        dir_name.push(((&row.1).1).id.to_owned());
        dir_name.push("boards");

        let result = get_file_names(dir_name);
        if result.is_ok() {
            let board_row = BoardRow {
                session: (row.1).0.clone(),
                session_user: (row.1).1.clone(),
                urls: result.unwrap()
            };

            board_rows.push(board_row);
        }
    }

    board_rows
}
