use sendgrid_api::SendGrid;
use std::env;

pub async fn send_email() {
    // Initialize the SendGrid client.
    //let SG_API_KEY = env::var("SENDGRID_API_KEY");
    println!("Inside Send Email!");
    //println!("{:?}", SG_API_KEY);
    let sendgrid = SendGrid::new_from_env();
    //let sendgrid = SendGrid(SENDGRID_API_KEY);

    // Send the email.
    sendgrid
        .send_mail(
            "email subject".to_string(),
            "body of email".to_string(),
            vec!["ganeshskandha@gmail.com".to_string()],
            vec!["ganeshskandha@gmail.com".to_string()],
            vec![],
            "ganeshskandha@gmail.com".to_string(),
        )
        .await;

    println!("successfully sent the email!");
}
