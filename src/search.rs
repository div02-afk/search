use crate::loader::loader;
use console::style;
use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct SearchResult {
    items: Option<Vec<SearchItem>>, // Use Option to handle potential absence of items
}

#[derive(Deserialize)]
struct SearchItem {
    title: String,
    link: String,
    snippet: String,
}

pub async fn google_search(query: String, count: u8) -> Result<String, Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("GOOGLE_CONSOLE_API_KEY")?;
    let cx = env::var("GOOGLE_CUSTOM_SEARCH_ENGINE_ID")?;

    let url = format!(
        "https://www.googleapis.com/customsearch/v1?q={}&key={}&cx={}",
        query, api_key, cx
    );

    let (tx, spinner_handle) = loader().await;

    let client = Client::new();
    let response = client.get(&url).send().await?;
    let search_result: SearchResult = response.json().await?;

    tx.send(true)?;
    spinner_handle.await?;

    println!();
    if let Some(items) = search_result.items {
        let mut result_count = count.min(items.len() as u8);

        for item in items.into_iter().take(result_count as usize) {
            let hyperlink = format!("\x1b]8;;{0}\x1b\\{1}\x1b]8;;\x1b\\", item.link, item.title);
            println!("{}", style(hyperlink).cyan().blink());
            println!("{}", item.snippet);
            println!();
        }
    } else {
        println!("No results found.");
    }

    Ok("".to_string())
}
