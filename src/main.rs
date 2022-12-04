mod lex;
mod parser;
use std::env;
use std::fs;

use crate::lex::*;

fn main() {
    // get file path from args
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mode = &args[2];

    // get content from file
    let content = fs::read_to_string(file_path).expect("err reading file");
    // get tokens
    let mut tokenizer = Tokenizer::new(content);
    match mode.as_str() {
        "lex" => {
            lex::inspect(&mut tokenizer);
        }
        "parse" => {
            parser::parse(&mut tokenizer).unwrap();
        }
        _ => {
            println!("mode not found");
        }
    }
    // inspect(&mut tokenizer);
}
