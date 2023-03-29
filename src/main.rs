use inquire::Text;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let _message = Text::new("").with_help_message("").prompt().unwrap();

    println!("Your input: {}", _message);

    let body = reqwest::get("https://example.com").await?.text().await?;

    println!("Body: {:?}", body);

    Ok(())
}
