// use inquire::Text;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
struct ChatRequest {
    model: String,
    prompt: String,
    max_tokens: i32,
    temperature: i32,
}

// {
//     "id": "cmpl-uqkvlQyYK7bGYrRHQ0eXlWi7",
//     "object": "text_completion",
//     "created": 1589478378,
//     "model": "text-davinci-003",
//     "choices": [
//       {
//         "text": "\n\nThis is indeed a test",
//         "index": 0,
//         "logprobs": null,
//         "finish_reason": "length"
//       }
//     ],
//     "usage": {
//       "prompt_tokens": 5,
//       "completion_tokens": 7,
//       "total_tokens": 12
//     }
//   }

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    // let _message = Text::new("").with_help_message("").prompt().unwrap();

    let request_json = ChatRequest {
        model: String::from("text-davinci-003"),
        prompt: String::from("Say this is a test"),
        max_tokens: 7,
        temperature: 0,
    };
    let request_json = serde_json::to_string(&request_json).unwrap();

    // println!("Your input: {}", _message);

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.openai.com/v1/completions")
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

    println!("{}", res);

    Ok(())
}
