use self::tokenize::gen_token;
use crate::lex::{dfa::get_token, table::DfaState as state, table::Token};
use colored::*;

pub fn tokenize(pre_processed: Vec<String>) -> Vec<Result<Vec<Token>, String>> {
    let mut result_tokens = Vec::new();
    for line in pre_processed {
        let mut token_in_this_line: Vec<Token> = Vec::new();
        let mut buffer = String::from("");
        let mut current_state = state::Start;
        let mut has_err_this_line = 0;

        let line = line.trim_start();

        for (index, current_char) in line.chars().enumerate() {
            let origin_state = current_state.clone();
            get_token(&mut current_state, current_char);

            if let state::Start = current_state {
                get_token(&mut current_state, current_char);
                let token = gen_token(origin_state, buffer.clone());
                match token {
                    None => {}
                    Some(tok) => token_in_this_line.push(tok),
                }
                buffer.clear();
            }

            if let state::ErrFirst = current_state {
                has_err_this_line = index + 1;
                // break;
            }
            match (current_state, current_char) {
                (state::StringStartNow, ' ') => buffer.push(' '),
                (_, ' ') => {}
                (_, _) => buffer.push(current_char),
            }
        }
        if has_err_this_line == 0 {
            result_tokens.push(Ok(token_in_this_line));
        } else {
            result_tokens.push(Err(has_err_this_line.to_string()))
        }
    }
    result_tokens
}

pub fn show_tokens(tokens: Vec<Result<Vec<Token>, String>>) {
    for (index, line) in tokens.iter().enumerate() {
        print!("line{}: ", index + 1);
        match line {
            Err(index) => {
                print!(
                    "{} {}",
                    "Err at this line, postion".bright_red(),
                    (*index).bright_red()
                )
            }
            _ => {
                for token in line.as_ref().unwrap() {
                    match *token {
                        Token::BooleanLitral(_) => print!("{}, ", token.to_string().green()),
                        Token::Identifier(_) => print!("{}, ", token.to_string().bright_white()),
                        Token::Numbers(_) => print!("{}, ", token.to_string().purple()),
                        Token::Reserved(_) => print!("{}, ", token.to_string().bright_yellow()),
                        Token::Symbols(_) => print!("{}, ", token.to_string().bright_blue()),
                        Token::StringLitral(_) => print!("{}, ", token.to_string().purple()),
                    }
                }
            }
        }
        println!()
    }
}

mod dfa;
mod table;
mod tokenize;
