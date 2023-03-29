use inquire::Text;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ReqMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReqMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatResponse {
    id: String,
    object: String,
    created: i32,
    model: String,
    choices: Vec<ResChoice>,
    usage: ResUsage,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResChoice {
    index: i32,
    message: ResMessage,
    finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResUsage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

async fn chat_once() -> reqwest::Result<()> {
    let _message = Text::new("").with_help_message("").prompt().unwrap();

    let request_json = ChatRequest {
        model: String::from("gpt-3.5-turbo"),
        messages: vec![ReqMessage {
            role: String::from("user"),
            content: _message,
        }],
    };
    let request_json = serde_json::to_string(&request_json).unwrap();

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header(CONTENT_TYPE, "application/json")
        .header(
            AUTHORIZATION,
            format!("Bearer {}", env::var("OPENAI_API_KEY").unwrap()),
        )
        .body(request_json)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let res_json: ChatResponse = serde_json::from_str(&res).unwrap();
    let res_str = res_json
        .choices
        .iter()
        .map(|v| v.message.content.clone())
        .map(|v| v.replace("\n\n", ""))
        .collect::<Vec<_>>()
        .join("");

    println!("{}", res_str);
    Ok(())
}

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    loop {
        chat_once().await.expect("end")
    }
}
