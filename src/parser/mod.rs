mod dfa;
mod states;
use dfa::*;

pub fn parse(tokens: Vec<Token>) {
    let mut tokens = tokens.clone();
    tokens.push(Token::Symbols("$".to_string()));
    let mut inp = tokens.iter();

    let mut stack = vec![WState::Prog];
    let mut this_char = inp.next().unwrap();

    // let mut node = Node::new(WState::Prog);

    while !stack.is_empty() {
        println!("============");
        dbg!(&stack);
        dbg!(this_char);
        if let WState::Terminal(cha) = stack.last().unwrap() {
            if cha == this_char {
                stack.pop();
                this_char = inp.next().unwrap();
            }
        } else {
            let new_state = trans(stack.last().unwrap(), this_char.clone()).unwrap();
            stack.pop();
            let first_1 = new_state.first().unwrap().clone();
            // node.push(&new_state);
            if let WState::Empty = first_1 {
            } else {
                for i in new_state.iter().rev() {
                    stack.push(i.clone());
                }
            }
        }
    }
    // node.print(0);
}

use crate::lex::Token;

use self::states::WState;
