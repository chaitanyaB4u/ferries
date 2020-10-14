use chrono::NaiveDateTime;

use crate::models::enrollments::ManagedEnrollmentRequest;
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
}

const ENROLLMENT_SENDER_ID: &'static str = "enrollment@krscode.com";
const OUT: &'static str = "out";
const TO: &'static str = "to";
const CC: &'static str = "cc";
const PENDING: &'static str = "pending";

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
}

impl MailOut {
    pub fn for_enrollment(request: &ManagedEnrollmentRequest, enrollment_id: &str) -> MailOut {
        let fuzzy_id = util::fuzzy_id();

        MailOut {
            id: fuzzy_id,
            from_user_id: request.coach_id.to_owned(),
            program_id: request.program_id.to_owned(),
            enrollment_id: enrollment_id.to_owned(),
            from_email: ENROLLMENT_SENDER_ID.to_owned(),
            subject: request.subject.to_owned(),
            content: Some(request.message.to_owned()),
            in_out: OUT.to_owned(),
            status: PENDING.to_owned(),
            reply_to: " ".to_owned(),
            error: " ".to_owned(),
            to_send_on: util::now(),
        }
    }
}

#[derive(Queryable, Debug, Identifiable, Insertable)]
#[table_name = "mail_recipients"]
pub struct MailRecipient {
    pub id: String,
    pub correspondence_id: String,
    pub to_user_id: Option<String>,
    pub to_email: String,
    pub to_type: String,
}

impl MailRecipient {
    pub fn for_enrollment(member: &User, coach: &User, correspondence_id: &str) -> Vec<MailRecipient> {
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
