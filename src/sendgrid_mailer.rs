use sendgrid::v3::{Attachment, Content, Disposition, Email, Message, Personalization, Sender};
use std::env;
use chrono::prelude::*;

pub fn mailer_sg(
    mail_body: &str,
    recording_base_64: &str,
    subject: String,
) -> Result<reqwest::blocking::Response, sendgrid::SendgridError> {

    // read config vars
    let from_mail = env::var("EMAIL_FROM").expect("Failed to get env var EMAIL_FROM");
    let reply_to_mail = from_mail.clone();
    let to_mail = env::var("EMAIL_TO").expect("Failed to get env var EMAIL_TO");
    let key = env::var("SENDGRID_API_KEY").expect("Failed to get SENDGRID_API_KEY");
    let mail_body = mail_body.to_string();
    let recording_base_64 = recording_base_64.to_string();
    let sg = Sender::new(key);

    // Create an attachment from the MP3 data (base64)
    let attachment = Attachment::new()
        .set_base64_content(recording_base_64)
        .set_disposition(Disposition::Attachment)
        .set_filename("Recording.mp3".to_string())
        .set_mime_type("audio/mp3");

    // Set destination and content
    let too = Personalization::new(Email::new(to_mail));
    let c = Content::new()
        .set_content_type("text/html")
        .set_value(mail_body);

    // Create the email message
    let message = Message::new(Email::new(from_mail))
        .set_subject(&subject)
        .set_reply_to(Email::new(reply_to_mail))
        .add_personalization(too)
        .add_content(c)
        .add_attachment(attachment);

    // Send the email using the SendGrid API
    sg.send(&message)
}

pub fn send_sendgrid_mail(formatted_body: String, recording: String) {
    
    // send mail via sendgrid api, retry on error.
    loop {
        let local: DateTime<Local> = Local::now();
        let subject = format!("Ai Newsie - {}", local.date_naive().to_string());
        match mailer_sg(&formatted_body, &recording, subject) {
            Ok(_) => println!("Email Sent!"),
            Err(e) => {
                eprintln!("Error Sending Sengrid request: [{e}]. Retrying.");
                continue;
            }
        }
        break;
    }
}
