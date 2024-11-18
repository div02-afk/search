use crate::loader::loader;
use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::env;

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
    usageMetadata: UsageMetadata,
    modelVersion: String,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: Content,
    finishReason: String,
    avgLogprobs: f64,
}

#[derive(Debug, Deserialize)]
struct Content {
    parts: Vec<Part>,
    role: String,
}

#[derive(Debug, Deserialize)]
struct Part {
    text: String,
}

#[derive(Debug, Deserialize)]
struct UsageMetadata {
    promptTokenCount: u32,
    candidatesTokenCount: u32,
    totalTokenCount: u32,
}

pub async fn gemini_chat(query: String) -> Result<String, Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("GEMINI_API_KEY")?;
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
    api_key);

    let body = json!({
        "contents": [
            {
                "parts": [
                    {
                        "text": query
                    }
                ]
            }
        ]
    });

    let (tx, spinner_handle) = loader().await;
    let client = Client::new();

    // Send the POST request
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;
    let response_text: GeminiResponse = response.json().await?;

    tx.send(true)?;
    spinner_handle.await?;

    println!(
        "{}",
        response_text.candidates[0].content.parts[0].text.clone()
    );
    Ok("".to_string())
}
