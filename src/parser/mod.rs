mod dfa;
mod states;
use dfa::*;
mod ast;
use ast::*;

pub fn parse(tokenizer: &mut Tokenizer) -> Result<(), ()> {
    let mut stack = vec![WState::Prog];

    let mut ast = Node::new(WState::Prog);
    let mut current_token = tokenizer.get_next_token().unwrap();

    while !stack.is_empty() {
        println!("=============================");
        println!("栈中：");
        println!("{:?}", &stack);
        println!("剩余的字符串：");
        println!("{:?}", current_token);
        if let WState::Terminal(cha) = stack.last().unwrap() {
            if *cha == current_token
                || (matches!(cha, Token::Identifier(_))
                    && matches!(current_token, Token::Identifier(_)))
            {
                stack.pop();
                current_token = tokenizer.get_next_token().unwrap();
            } else {
                println!();
                tokenizer.print_err((
                    tokenizer.get_current_line(),
                    tokenizer.current_location_inline,
                    ErrType::Null,
                ));
                println!("Expected {:?}, but got {:?}", cha, current_token);
                println!();
                return Err(());
            }
        } else {
            let new_state = trans(stack.last().unwrap(), current_token.clone()).unwrap();
            let old = stack.pop().unwrap();
            let first_1 = new_state.first().unwrap().clone();
            ast.push(&new_state);
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
    ast.print(0);
    Ok(())
    // node.print(0);
}

use crate::lex::{ErrType, Token, Tokenizer};

use self::states::WState;
