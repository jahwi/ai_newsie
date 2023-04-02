use ai_newsie::{get_articles, get_openai_response, get_polly, send_sendgrid_mail};
use serde_json::{Value, json};
use lambda_runtime::{service_fn, LambdaEvent, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(run_bot);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub async fn run_bot(_event: LambdaEvent<Value>) -> Result<Value, Error>  {

    // Get topics from env, or else use default topics "news, ai, and climate."
    let topics = std::env::var("AI_BOT_TOPICS").unwrap_or("news,ai,climate".to_string());
    let topics: Vec<&str> = topics.split(",").collect();

    // get articles
    println!("Grabbing Articles...");
    let articles = get_articles(topics).await;

    println!("Querying GPT-3.5-TURBO");
    let ret = get_openai_response(articles).await;

    // Format response in pretty HTML
    let article_mail_body = ret.replace("\\n", "<br>").replace("\"", "");
    let formatted_body = format!(
        r#"<html><head></head><body><div align="justify" style="background-color: rgb(55, 63, 58); color: rgb(177, 247, 226); padding: 0px 20px;">{}</div></body></html>"#,
        article_mail_body
    );

    // Send to AWS Polly
    let ret = ret.replace("\\n", "\n");
    println!("Sending to Polly...");
    let recording = get_polly(ret).await;
 
    // Send mail
    println!("Sending mail...");
    send_sendgrid_mail(formatted_body, recording);

    Ok(json!({"Result":"Success"}))
}