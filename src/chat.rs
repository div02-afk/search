use crate::loader::loader;
use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::env;

#[derive(Deserialize, Debug)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
    usageMetadata: UsageMetadata,
    modelVersion: String,
}

#[derive(Deserialize, Debug)]
struct Candidate {
    content: Content,
    finishReason: String,
    safetyRatings: Vec<SafetyRating>,
    avgLogprobs: f64,
}

#[derive(Deserialize, Debug)]
struct Content {
    parts: Vec<Part>,
    role: String,
}

#[derive(Deserialize, Debug)]
struct Part {
    text: String,
}

#[derive(Deserialize, Debug)]
struct SafetyRating {
    category: String,
    probability: String,
}

#[derive(Deserialize, Debug)]
struct UsageMetadata {
    promptTokenCount: u32,
    candidatesTokenCount: u32,
    totalTokenCount: u32,
}

pub async fn gemini_chat(query: String) -> Result<String, Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("GEMINI_API_KEY")?;
    let url =
        format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-8b:generateContent?key={}", api_key);

    let body = json!({
        "contents": [
            {
                "parts": [
                    {
                        "text": query
                    }
                ]
            }
        ],
        "generationConfig": {
            "maxOutputTokens":1200,
        },
        "system_instruction": {
            "parts":{
                "text": "You are simpleSearch assistant, summarise/describe/define the input for me, keep it brief and factual"}
            },
        "model":"gemini-1.5-flash-8b"
    });

    let (tx, spinner_handle) = loader("Summary".to_string()).await;
    let client = Client::new();

    // Send the POST request

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    // let response_body = response.text().await?;
    // println!("Raw Response Body: {}", response_body);
    let response_text: GeminiResponse = response.json().await?;
    // println!("{:?}", response_text);

    tx.send(true)?;
    spinner_handle.await?;

    println!(
        "\n{}",
        response_text.candidates[0].content.parts[0].text.clone()
    );
    Ok("".to_string())
}
