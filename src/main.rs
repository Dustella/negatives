use std::env;
use std::fs;

fn main() {
    let args:Vec<String> = env::args().collect();
    let file_path = &args[1];
    let content = fs::read_to_string(file_path).expect("err reading file");
    let processed = pre::pre_process(content);
    dbg!(processed);
    lex::test();
}

mod lex;
mod pre;
