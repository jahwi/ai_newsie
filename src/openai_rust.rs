use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Value};
use std::env;

pub async fn query_openai(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    
    // Get key
    let key = env::var("OPENAI_API_KEY").expect("Failed to get OPENAI API KEY.");

    // Set request Headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(AUTHORIZATION, format!("Bearer {}", key).parse().unwrap());

    // Set request Body
    let params = json!({
        "model": "gpt-3.5-turbo",
        "messages": [{"role": "user", "content": format!("{}", prompt)}],
        "temperature": 0.7
    });

    // Send request
    let client = reqwest::Client::new();
    let resp = client
        .post("https://api.openai.com/v1/chat/completions")
        .headers(headers)
        .json(&params)
        .send()
        .await?;

    let resp: Value = resp.json().await?;
    let ret = resp["choices"][0]["message"]["content"].to_string();
    Ok(ret)
}

pub async fn get_openai_response(articles: String) -> String {
    loop {
        match query_openai(format!("Transform the following story chunks into a newsletter format. Each story is composed of a Snippet and a leading paragraph. It should be fit for transcribing into audio form, and interesting for daily consumption. It is a personal newsletter and should be as if written by an expert newsletter writer named AI Newsie. Do not include a closing salutation, and make the newsletter as long as possible. This is the text: {articles}")).await
        {
            Ok(val) => break val,
            Err(e) => {
                eprintln!("Error querying OpenAI: [{e}]. Retrying.");
                continue;
            }
        }
    }
}
