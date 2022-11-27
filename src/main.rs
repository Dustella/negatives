use std::env;
use std::fs;

use lex::show_tokens;

use crate::lex::tokenize;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let content = fs::read_to_string(file_path).expect("err reading file");
    let processed = pre::pre_process(content);
    let tokens = tokenize(processed.clone());
    show_tokens(tokens, processed);
}

mod lex;
mod parser;
mod pre;
