use super::{Token, Tokenizer};
use colored::*;

// test: this will inspect the next token and print it for debug
pub fn inspect(tokenizer: &mut Tokenizer) {
    println!(
        "line {}: {}",
        tokenizer.current_line + 1,
        tokenizer.get_current_line().trim()
    );
    let mut current_line = tokenizer.current_line.clone();

    while !tokenizer.finished {
        let token = tokenizer.get_next_token();
        if current_line != tokenizer.current_line {
            println!();
            println!(
                "line {}: {}",
                tokenizer.current_line + 1,
                tokenizer.get_current_line().trim()
            );
        }
        // println!("{}: {:?}", tokenizer.current_line, token);
        if let Ok(token) = token {
            match token {
                Token::Identifier(_) => print!("{}", token.to_string().bright_white()),
                Token::Constant(_) => print!("{}", token.to_string().purple()),
                Token::Reserved(_) => print!("{}", token.to_string().bright_yellow()),
                Token::Operator(_) => print!("{}", token.to_string().bright_blue()),
                Token::StringLiteral(_) => print!("{}", token.to_string().purple()),
            }
        } else {
        }
        current_line = tokenizer.current_line.clone();
    }
}
