use diesel::prelude::*;

use crate::commons::util;

use crate::models::enrollments::{Enrollment, PlanCriteria};
use crate::models::notes::Note;
use crate::models::sessions::Session;

use crate::schema::enrollments;
use crate::schema::session_notes;

use crate::schema::enrollments::dsl::*;
use crate::schema::session_notes::dsl::*;
use crate::schema::sessions::dsl::*;

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
