use diesel::prelude::*;

use crate::schema::correspondences::dsl::*;
use crate::schema::mail_recipients::dsl::*;

use crate::models::correspondences::{Correspondence, MailCriteria, MailOut, MailRecipient, Mailable};

const MAIL_CREATION_ERROR: &'static str = "Error in creating the invitation mail. But enrollment is done.";

pub type MailType = (Correspondence, Vec<MailRecipient>);
pub type MailResult = Result<Vec<MailType>, diesel::result::Error>;
pub type MailableResult = Result<Vec<Mailable>, diesel::result::Error>;

/**
 * Let us offer 3 pending mails and mark them as Marked for avoiding
 * repeat mails
 */

pub fn sendable_mails(connection: &MysqlConnection) -> MailableResult {
    let criteria = MailCriteria {
        status: "pending".to_owned(),
        in_out: "out".to_owned(),
    };

    let mailables: Vec<Mailable> = get_mails(connection, &criteria)?
        .into_iter()
        .map(|item| Mailable {
            correspondence: item.0,
            receipients: item.1,
        })
        .collect();

    let ids: Vec<&str> = mailables.iter().map(|item| item.correspondence.id.as_str()).collect();
    let query = correspondences.filter(crate::schema::correspondences::id.eq_any(ids));
    diesel::update(query).set(status.eq("marked")).execute(connection)?;

    Ok(mailables)
}

pub fn get_mails(connection: &MysqlConnection, criteria: &MailCriteria) -> MailResult {
    let corres: Vec<Correspondence> = correspondences
        .filter(status.eq(criteria.status.as_str()))
        .filter(in_out.eq(criteria.in_out.as_str()))
        .order_by(created_at.asc())
        .limit(3)
        .load(connection)?;

    let people = MailRecipient::belonging_to(&corres).load::<MailRecipient>(connection)?.grouped_by(&corres);

    let mails = corres.into_iter().zip(people).collect::<Vec<_>>();

    Ok(mails)
}

pub fn create_mail(connection: &MysqlConnection, mail_out: MailOut, recipients: Vec<MailRecipient>) ->Result<usize,&'static str> {

    let result = diesel::insert_into(correspondences).values(mail_out).execute(connection);
    if result.is_err() {
        return Err(MAIL_CREATION_ERROR);
    }

    let result = diesel::insert_into(mail_recipients).values(recipients).execute(connection);
    if result.is_err() {
        return Err(MAIL_CREATION_ERROR);
    }

    Ok(result.unwrap())
}
