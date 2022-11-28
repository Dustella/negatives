mod dfa;
mod states;
use dfa::*;
mod ast;
use ast::*;

pub fn parse(tokens: Vec<Token>) -> Result<(), ()> {
    let mut tokens = tokens.clone();
    tokens.push(Token::Symbols("$".to_string()));
    let mut inp = tokens.iter();

    let mut stack = vec![WState::Prog];
    let mut this_char = inp.next().unwrap();

    let mut node = Node::new(WState::Prog);

    while !stack.is_empty() {
        println!("=============================");
        println!("栈中：");
        println!("{:?}", &stack);
        println!("剩余的字符串：");
        println!("{:?}{}", inp, this_char);
        if let WState::Terminal(cha) = stack.last().unwrap() {
            if cha == this_char
                || (matches!(cha, Token::Identifier(_))
                    && matches!(this_char, Token::Identifier(_)))
            {
                stack.pop();
                this_char = inp.next().unwrap();
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

use crate::lex::Token;

use self::states::WState;
