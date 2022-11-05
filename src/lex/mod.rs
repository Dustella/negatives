use crate::lex::{
    dfa::get_token,
    table::DfaState as state,
    table::{is_reversed, Token},
};

use self::tokenize::gen_token;

pub fn tokenize(pre_processed: Vec<String>) -> Vec<Vec<Token>> {
    let mut result_tokens = Vec::new();
    for line in pre_processed {
        let mut token_in_this_line: Vec<Token> = Vec::new();
        let mut buffer = String::from("");
        let mut current_state = state::Start;

        let line = line.trim_start();

        for current_char in line.chars() {
            // if current_char == ' ' {
            //     if let state::LetterNow = current_state {
            //     } else {
            //         continue;
            //     }
            // }

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
            match (current_state, current_char) {
                (state::StringStartNow, ' ') => buffer.push(' '),
                (_, ' ') => {}
                (_, _) => buffer.push(current_char),
            }
        }
        result_tokens.push(token_in_this_line);
    }
    result_tokens
}

pub fn show_tokens(tokens: Vec<Vec<Token>>) {
    for (index, line) in tokens.iter().enumerate() {
        print!("line{}: ", index + 1);
        for token in line {
            print!("{}, ", token);
        }
        println!()
    }
}

mod dfa;
mod table;
mod tokenize;
