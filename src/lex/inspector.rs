use super::{Token, Tokenizer};
use colored::*;

// test: this will inspect the next token and print it for debug
pub fn inspect(tokenizer: &mut Tokenizer) {
    let mut current_line = tokenizer.current_line.clone();
    while let Some(token) = tokenizer.get_next_token() {
        if current_line != tokenizer.current_line {
            println!();
            println!(
                "line {}: {}",
                tokenizer.current_line + 1,
                tokenizer.get_current_line().trim()
            );
        }
        // println!("{}: {:?}", tokenizer.current_line, token);
        let token = token;
        if let Ok(token) = token {
            match token {
                Token::Boolean(_) => print!("{}, ", token.to_string().green()),
                Token::Identifier(_) => print!("{}, ", token.to_string().bright_white()),
                Token::Numbers(_) => print!("{}, ", token.to_string().purple()),
                Token::Reserved(_) => print!("{}, ", token.to_string().bright_yellow()),
                Token::Punctuator(_) => print!("{}, ", token.to_string().bright_blue()),
                Token::Litral(_) => print!("{}, ", token.to_string().purple()),
            }
        } else {
            let token = token.err().unwrap();
            println!("{}: {:?}", tokenizer.current_line, token);
            println!(
                "{}{}:",
                "Error found at line".red(),
                tokenizer.current_line.to_string().red()
            );
            println!("{}", token.0.red());
            println!("{}^ <-here", "-".repeat(token.1).to_string());
            print!("{}", token.2.to_string());
            println!(" '{}'", tokenizer.get_last_char().escape_default());
        }
        current_line = tokenizer.current_line.clone();
    }
}
