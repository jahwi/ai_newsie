mod nyt_rs;
mod openai_rust;
mod sendgrid_mailer;
mod to_speech;
pub use nyt_rs::get_articles;
pub use openai_rust::get_openai_response;
pub use sendgrid_mailer::send_sendgrid_mail;
pub use to_speech::get_polly;
