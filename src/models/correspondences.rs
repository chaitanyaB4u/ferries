use serde::{Deserialize, Serialize};

use chrono::NaiveDateTime;

use crate::models::enrollments::ManagedEnrollmentRequest;
use crate::models::sessions::Session;
use crate::models::users::User;

use crate::schema::correspondences;
use crate::schema::mail_recipients;

use crate::commons::util;

#[derive(Queryable, Debug, Identifiable)]
pub struct Correspondence {
    pub id: String,
    pub from_user_id: String,
    pub program_id: String,
    pub enrollment_id: String,
    pub from_email: String,
    pub subject: String,
    pub content: Option<String>,
    pub in_out: String,
    pub status: String,
    pub sent_at: Option<NaiveDateTime>,
    pub reply_to: String,
    pub error: String,
    pub error_reason: Option<String>,
    pub to_send_on: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub mail_type: String,
}

const SCHEDULE_SENDER_ID: &str = "schedule@krscode.com";

const OUT: &str = "out";
const TO: &str = "to";
const CC: &str = "cc";

const PENDING: &str = "pending";

const NORMAL: &str = "normal";
const EVENT: &str = "event";

#[derive(Insertable)]
#[table_name = "correspondences"]
pub struct MailOut {
    pub id: String,
    pub from_user_id: String,
    pub program_id: String,
    pub enrollment_id: String,
    pub from_email: String,
    pub subject: String,
    pub content: Option<String>,
    pub in_out: String,
    pub status: String,
    pub reply_to: String,
    pub error: String,
    pub to_send_on: NaiveDateTime,
    pub mail_type: String,
}

impl MailOut {
    fn new(from_user_id: String, program_id: String, enrollment_id: String, subject: String, content: String, mail_type: &str) -> MailOut {
        let fuzzy_id = util::fuzzy_id();

        MailOut {
            id: fuzzy_id,
            from_email: SCHEDULE_SENDER_ID.to_owned(),
            from_user_id,
            program_id,
            enrollment_id,
            subject,
            content: Some(content),
            in_out: OUT.to_owned(),
            status: PENDING.to_owned(),
            reply_to: " ".to_owned(),
            error: " ".to_owned(),
            to_send_on: util::now(),
            mail_type: mail_type.to_owned(),
        }
    }

    pub fn for_enrollment(request: &ManagedEnrollmentRequest, enrollment_id: &str) -> MailOut {
        MailOut::new(
            request.coach_id.to_owned(),
            request.program_id.to_owned(),
            enrollment_id.to_owned(),
            request.subject.to_owned(),
            request.message.to_owned(),
            NORMAL,
        )
    }

    pub fn for_new_session(session: &Session, coach: &User, member: &User) -> MailOut {
        let content = FerrisEvent::new_session_event(session, coach, member);

        MailOut::new(
            coach.id.to_owned(),
            session.program_id.to_owned(),
            session.enrollment_id.to_owned(),
            session.name.to_owned(),
            content,
            EVENT,
        )
    }

    pub fn for_cancel_session(session: &Session, coach: &User, member: &User) -> MailOut {
        let content = FerrisEvent::cancel_event(session, coach, member);

        MailOut::new(
            coach.id.to_owned(),
            session.program_id.to_owned(),
            session.enrollment_id.to_owned(),
            session.name.to_owned(),
            content,
            EVENT,
        )
    }
}

#[derive(Queryable, Debug, Associations, Identifiable, Insertable)]
#[belongs_to(Correspondence)]
#[table_name = "mail_recipients"]
pub struct MailRecipient {
    pub id: String,
    pub correspondence_id: String,
    pub to_user_id: Option<String>,
    pub to_email: String,
    pub to_type: String,
}

impl MailRecipient {
    pub fn build_recipients(member: &User, coach: &User, correspondence_id: &str) -> Vec<MailRecipient> {
        let to_record = MailRecipient {
            id: util::fuzzy_id(),
            correspondence_id: correspondence_id.to_owned(),
            to_user_id: Some(member.id.to_owned()),
            to_email: member.email.to_owned(),
            to_type: TO.to_owned(),
        };

        let cc_record = MailRecipient {
            id: util::fuzzy_id(),
            correspondence_id: correspondence_id.to_owned(),
            to_user_id: Some(coach.id.to_owned()),
            to_email: coach.email.to_owned(),
            to_type: CC.to_owned(),
        };

        vec![to_record, cc_record]
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct MailCriteria {
    pub status: String,
    pub in_out: String,
}

#[juniper::object]
impl Correspondence {
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn from_email(&self) -> &str {
        self.from_email.as_str()
    }

    pub fn subject(&self) -> &str {
        self.subject.as_str()
    }

    pub fn content(&self) -> &str {
        match &self.content {
            Some(c) => c.as_str(),
            None => " ",
        }
    }

    pub fn mail_type(&self) -> &str {
        self.mail_type.as_str()
    }
}

#[juniper::object]
impl MailRecipient {
    pub fn to_type(&self) -> &str {
        self.to_type.as_str()
    }

    pub fn to_email(&self) -> &str {
        self.to_email.as_str()
    }
}

pub struct Mailable {
    pub correspondence: Correspondence,
    pub receipients: Vec<MailRecipient>,
}

#[juniper::object]
impl Mailable {
    pub fn correspondence(&self) -> &Correspondence {
        &self.correspondence
    }

    pub fn receipients(&self) -> &Vec<MailRecipient> {
        &self.receipients
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct FerrisEvent {
    pub id: String,
    pub description: Option<String>,
    pub organizer: Option<String>,
    pub attendee: Option<String>,
    pub sequence: i32,
    pub startDate: String,
    pub endDate: String,
    pub status: String,
    pub method: String,
}

impl FerrisEvent {
    fn new_session_event(session: &Session, coach: &User, member: &User) -> String {
        let start_date = session.original_start_date;
        let end_date = session.original_end_date;

        let event = FerrisEvent {
            id: session.id.to_owned(),
            sequence: 1,
            organizer: Some(coach.email.clone()),
            attendee: Some(member.email.clone()),
            description: session.description.clone(),
            startDate: util::format_time(&start_date),
            endDate: util::format_time(&end_date),
            status: "CONFIRMED".to_owned(),
            method: "REQUEST".to_owned(),
        };

        serde_json::to_string(&event).unwrap_or_else(|_|String::from(""))
    }

    fn cancel_event(session: &Session, coach: &User, member: &User) -> String {
        let start_date = session.original_start_date;
        let end_date = session.original_end_date;

        let event = FerrisEvent {
            id: session.id.to_owned(),
            sequence: 99,
            organizer: Some(coach.email.clone()),
            attendee: Some(member.email.clone()),
            description: session.closing_notes.clone(),
            startDate: util::format_time(&start_date),
            endDate: util::format_time(&end_date),
            status: "CANCELLED".to_owned(),
            method: "CANCEL".to_owned(),
        };

        serde_json::to_string(&event).unwrap_or_else(|_| String::from(""))
    }
}
