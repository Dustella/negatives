mod dfa;
mod states;
#![recursion_limit = "25565"]
fn main() {
    let mut raw = String::from("");
    let stdin = std::io::stdin();
    stdin.read_line(&mut raw).unwrap();
    let mut tmp: String = raw.chars().collect();
    tmp = tmp.replace('\n', "");
    tmp = tmp.replace('\r', "");
    tmp.push('$');
    let mut inp = tmp.chars();

    let mut stack = vec![table::State::E];
    let mut this_char = inp.next().unwrap();

    let mut node = Node::new(State::E);

    while !stack.is_empty() {
        println!("============");
        dbg!(&stack);
        dbg!(this_char);
        if let State::Terminal(cha) = stack.last().unwrap() {
            if *cha == this_char {
                stack.pop();
                this_char = inp.next().unwrap();
            }
        } else {
            let new_state = trans(stack.last().unwrap(), this_char).unwrap();
            stack.pop();
            let first_1 = new_state.first().unwrap().clone();
            node.push(&new_state);
            if let State::Eps = first_1 {
            } else {
                for i in new_state.iter().rev() {
                    stack.push(*i);
                }
            }
        }
    }
    node.print(0);
}
mod trans;

use table::State;
use trans::*;
mod ast;
mod table;
use ast::*;
