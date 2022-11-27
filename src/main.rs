use std::env;
use std::fs;

use lex::show_tokens;
use lex::Token;

use crate::lex::tokenize;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let content = fs::read_to_string(file_path).expect("err reading file");
    let processed = pre::pre_process(content);
    let tokens = tokenize(processed.clone());
    show_tokens(&tokens, processed);
    // flaten the tokens vec
    let mut new_tokens = vec![];
    for i in tokens {
        for j in i.unwrap() {
            new_tokens.push(j);
        }
    }
    parser::parse(new_tokens);
}

mod lex;
mod parser;
mod pre;
