use actix_web::client::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Personalizations {
    pub to: Option<Vec<HashMap<String, String>>>,
    pub cc: Option<Vec<HashMap<String, String>>>,
    pub bcc: Option<Vec<HashMap<String, String>>>,
}

impl Personalizations {
    pub fn as_personal_map(emails: &Vec<String>) -> Option<Vec<HashMap<String, String>>> {
        if emails.len() == 0 {
            return None;
        }

        let mut result = Vec::new();
        for email in emails {
            let mut map = HashMap::new();
            map.insert(String::from("email"), email.clone());

            result.push(map);
        }

        Some(result)
    }

    pub fn new(to_emails: &Vec<String>, cc_emails: &Vec<String>, bcc_emails: &Vec<String>) -> [Personalizations; 1] {
        let to = Personalizations::as_personal_map(to_emails);
        let cc = Personalizations::as_personal_map(cc_emails);
        let bcc = Personalizations::as_personal_map(bcc_emails);

        let personalization = Personalizations { to, cc, bcc };

        [personalization]
    }
}

#[derive(Serialize, Deserialize)]
pub struct Mail {
    pub from: HashMap<String, String>,
    pub personalizations: [Personalizations; 1],
    pub subject: String,
    pub content: [HashMap<String, String>; 1],
}

impl Mail {
    pub fn new(from: &str, to_emails: &Vec<String>, cc_emails: &Vec<String>, bcc_emails: &Vec<String>, subject: &str, body: &str) -> Mail {
        let mut from_map = HashMap::new();
        from_map.insert(String::from("email"), String::from(from));

        let personalizations = Personalizations::new(to_emails, cc_emails, bcc_emails);

        let mut body_map = HashMap::new();
        body_map.insert(String::from("type"), String::from("text/html"));
        body_map.insert(String::from("value"), String::from(body));

        let content = [body_map; 1];

        Mail {
            from: from_map,
            personalizations: personalizations,
            subject: String::from(subject),
            content: content,
        }
    }
}


pub async fn send_email(mail: &Mail) -> Result<String,String>{

    let mail_data = serde_json::to_string(&mail).unwrap();

    let api_key = env::var("SENDGRID_API_KEY").expect("The Sendgrid API Key should be set");
    let sendgrid_url = env::var("SENDGRID_URL").expect("The Sendgrid URL is not set");
    
    let auth = format!("{} {}", "Bearer", api_key);
    let sendgrid_client = Client::default();

    let response = sendgrid_client
        .post(sendgrid_url)
        .header("Authorization", auth)
        .header("Content-Type", "application/json")
        .send_body(&mail_data)
        .await;

    
    if response.is_ok() {
        return Ok(String::from("Ok"));
    }

    Err(String::from("Error in Mail Dispatch"))
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_rt;

    #[actix_rt::test]
    pub async fn test_should_send_mail() {
        println!("Calling Test Mail Dispatch Method");

        let from = "test@krscode.com";

        let to_emails = vec![String::from("krsmanian1972@gmail.com")];

        let cc_emails: Vec<String> = Vec::new();
        let bcc_emails: Vec<String> = Vec::new();

        let subject = "Unit Testing the API with Html Content";
        let content = test_html_body();

        let mail = Mail::new(from, &to_emails, &cc_emails, &bcc_emails, subject, content.as_str());

        let result = send_email(&mail).await;

        assert_eq!("Ok", result.unwrap());
    }

    fn test_html_body() -> String {
        let content = r#"
        <html>
            <body>
                <h5>Welcome to Ferris - The Coaching Assistant</h5>
                <br/>
                <p>Hundreds of companies around the world are using Rust in production today for fast, low-resource, cross-platform solutions. Software you know and love, like&nbsp;<a href="https://hacks.mozilla.org/2017/08/inside-a-super-fast-css-engine-quantum-css-aka-stylo/" rel="noopener noreferrer" target="_blank">Firefox</a>,&nbsp;<a href="https://blogs.dropbox.com/tech/2016/06/lossless-compression-with-brotli/" rel="noopener noreferrer" target="_blank">Dropbox</a>, and&nbsp;<a href="https://blog.cloudflare.com/cloudflare-workers-as-a-serverless-rust-platform/" rel="noopener noreferrer" target="_blank">Cloudflare</a>, uses Rust.&nbsp;<strong>From startups to large corporations, from embedded devices to scalable web services, Rust is a great fit.</strong></p><p><br></p><p>My biggest compliment to Rust is that it's boring, and this is an amazing compliment.</p><p class="ql-align-right">– Chris Dickinson, Engineer at npm, Inc</p><p class="ql-align-center"><a href="https://www.npmjs.com/" rel="noopener noreferrer" target="_blank"><img src="https://www.rust-lang.org/static/images/user-logos/npm.svg"></a></p><p class="ql-align-center"><a href="https://www.youtube.com/watch?v=u6ZbF4apABk" rel="noopener noreferrer" target="_blank"><img src="https://www.rust-lang.org/static/images/user-logos/yelp.png"></a></p><p>All the documentation, the tooling, the community is great - you have all the tools to succeed in writing Rust code.</p><p class="ql-align-right">– Antonio Verardi, Infrastructure Engineer</p><p><br></p>
            </body>
        </html>
        "#;

        String::from(content)
    }
}
