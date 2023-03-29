use inquire::{error::InquireResult, Text};

fn main() -> InquireResult<()> {
    let _message = Text::new("").with_help_message("").prompt()?;

    println!("Your input: {}", _message);

    Ok(())
}
