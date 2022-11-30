use std::env;
use std::fs;

use lex::show_tokens;
// use lex::Token;

use crate::lex::*;

fn main() {
    // get file path from args
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // get content from file
    let content = fs::read_to_string(file_path).expect("err reading file");
    // get tokens
    let mut tokenizer = Tokenizer::new(content);
    inspect(&mut tokenizer);
    // debug: print tokens
    // show_tokens(&tokens, processed);
    // flaten the tokens vec
    // let mut new_tokens = vec![];
    // for i in tokens {
    //     for j in i.unwrap() {
    //         new_tokens.push(j);
    //     }
    // }
    // // do parse
    // parser::parse(new_tokens).unwrap();
}

mod lex;
mod parser;
