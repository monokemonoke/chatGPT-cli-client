mod config;

use inquire::Text;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use spinners::Spinner;

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

impl Clone for ReqMessage {
    fn clone(&self) -> Self {
        ReqMessage {
            role: self.role.clone(),
            content: self.content.clone(),
        }
    }
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

async fn chat_once(messages: &Vec<ReqMessage>) -> reqwest::Result<ChatResponse> {
    let request_json = ChatRequest {
        messages: messages.to_vec(),
        model: String::from("gpt-3.5-turbo"),
    };
    let request_json = serde_json::to_string(&request_json).unwrap();

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", config::OPENAI_API_KEY))
        .body(request_json)
        .send()
        .await?
        .text()
        .await?;

    let res_json: ChatResponse = serde_json::from_str(&res).unwrap();
    Ok(res_json)
}

async fn chat() -> reqwest::Result<()> {
    let mut msgs: Vec<ReqMessage> = Vec::new();

    loop {
        let msg = Text::new("").with_help_message("").prompt();
        if let Err(_) = msg {
            break;
        }

        if let Ok(msg) = msg {
            msgs.push(ReqMessage {
                content: msg,
                role: "user".to_string(),
            });
        }

        let mut sp = Spinner::new(
            spinners::Spinners::Dots,
            "waiting for the responses...".into(),
        );
        let res = chat_once(&msgs).await;
        sp.stop();
        print!("\r                             \r");
        if let Err(_) = res {
            break;
        }

        if let Ok(res) = res {
            println!(
                "{}",
                res.choices
                    .iter()
                    .map(|v| v.message.content.clone())
                    .collect::<Vec<_>>()
                    .join("")
            );

            msgs.extend(res.choices.iter().map(|v| ReqMessage {
                role: v.message.role.clone(),
                content: v.message.content.clone(),
            }));
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    chat().await
}
