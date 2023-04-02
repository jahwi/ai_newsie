use serde::{Deserialize, Serialize};
use std::env;
use futures::future::try_join_all;

#[derive(Debug, Serialize, Deserialize)]
struct Story {
    snippet: String,
    lead_paragraph: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Docs {
    docs: Vec<Story>,
}

#[derive(Debug, Serialize, Deserialize)]
struct NYT {
    status: String,
    response: Docs,
}

pub async fn search_nyt(query: String) -> Result<String, Box<dyn std::error::Error>> {
    
    // get nyt api key from env
    let key = env::var("NYT_API_KEY").expect("Could not get NYT API key.");

    // query nyt api
    let resp = reqwest::get(format!("https://api.nytimes.com/svc/search/v2/articlesearch.json?q={query}&sort=newest&api-key={key}"))
        .await?
        .json::<NYT>()
        .await?;

    // format a bit for prompting purposes
    let ret_str: String = resp
        .response
        .docs
        .iter()
        .take(5)
        .map(|story| {
            format!(
                "Snippet: {}. Lead Paragraph: {}\n",
                story.snippet, story.lead_paragraph
            )
        })
        .collect();

    Ok(ret_str)
}

pub async fn get_articles(topics: Vec<&str>) -> String {
    // query nyt api for each topic, retry on failure
    loop {
        let _articles = match try_join_all(
            topics
                .iter()
                .map(|topic| search_nyt(String::from(*topic)))
        )
        .await
        {
            Ok(art) =>  break art.concat(),
            Err(error) => {
                eprintln!("Error searching NYT: [{}]. Retrying.", error);
                continue;
            }
        };
    }
}
