mod dfa;
mod states;
use dfa::*;
mod ast;
use ast::*;

pub fn parse(tokenizer: &mut Tokenizer) -> Result<(), ()> {
    let mut stack = vec![WState::Prog];

    let mut node = Node::new(WState::Prog);
    let mut this_char = tokenizer.get_next_token().unwrap().unwrap();

    while tokenizer.finished == false {
        println!("=============================");
        println!("栈中：");
        println!("{:?}", &stack);
        println!("剩余的字符串：");
        println!("{:?}", this_char);
        if let WState::Terminal(cha) = stack.last().unwrap() {
            if *cha == this_char
                || (matches!(cha, Token::Identifier(_))
                    && matches!(this_char, Token::Identifier(_)))
            {
                stack.pop();
                this_char = tokenizer.get_next_token().unwrap().unwrap();
            } else {
                return Err(());
            }
        } else {
            let new_state = trans(stack.last().unwrap(), this_char.clone()).unwrap();
            let old = stack.pop().unwrap();
            let first_1 = new_state.first().unwrap().clone();
            node.push(&new_state);
            println!("新的产生式:");
            println!("{:?} -> {:?}", old, new_state);
            if let WState::Empty = first_1 {
            } else {
                for i in new_state.iter().rev() {
                    stack.push(i.clone());
                }
            }
        }
    }
    node.print(0);
    Ok(())
    // node.print(0);
}

use crate::lex::{Token, Tokenizer};

use self::states::WState;
