// use inquire::Text;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use std::env;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    // let _message = Text::new("").with_help_message("").prompt().unwrap();

    let json_request = format!(
        r#"{{
            "model": "text-davinci-003",
            "prompt": "Say this is a test",
            "max_tokens": 7,
            "temperature": 0
        }}"#
    );

    // println!("Your input: {}", _message);

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.openai.com/v1/completions")
        .header(CONTENT_TYPE, "application/json")
        .header(
            AUTHORIZATION,
            format!("Bearer {}", env::var("OPENAI_API_KEY").unwrap()),
        )
        .body(json_request)
        .send()
        .await
        .unwrap();

    println!("{}", res.text().await.unwrap());

    Ok(())
}
