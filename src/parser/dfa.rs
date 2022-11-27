use std::f32::consts::E;

use super::super::lex::Token;
use super::states::WState;

pub fn trans(state: &WState, cha: Token) -> Result<Vec<WState>, String> {
    match state {
        WState::Expr => match cha {
            Token::Symbols(sym) => {
                if sym == "(" {
                    Ok(vec![WState::Term, WState::Expr])
                } else {
                    Err("Expected '(', found ".to_string() + &sym)
                }
            }
            Token::Numbers(_) => Ok(vec![WState::Term, WState::Expr]),
            _ => Err("".to_string()),
        },
        WState::Term => match cha {
            // '(' => Ok(vec![WState::Factor, WState::Termt]),
            Token::Symbols(sym) => {
                if sym == "(" {
                    Ok(vec![WState::Factor, WState::Termt])
                } else {
                    Err("Expected '(', found ".to_string() + &sym)
                }
            }
            // 'n' => Ok(vec![WState::Factor, WState::Termt]),
            Token::Numbers(_) => Ok(vec![WState::Factor, WState::Termt]),
            _ => Err("".to_string()),
        },
        WState::Factor => match cha {
            // Token::Symbols(String::from('(')) => Ok(vec![
            //     WState::Terminal(Token::Symbols("(".to_string())),
            //     WState::Expr,
            //     WState::Terminal(Token::Symbols(")".to_string())),
            // ]),
            Token::Symbols(sym) => {
                if sym == "(" {
                    Ok(vec![
                        WState::Terminal(Token::Symbols("(".to_string())),
                        WState::Expr,
                        WState::Terminal(Token::Symbols(")".to_string())),
                    ])
                } else {
                    Err("Expected '(', found ".to_string() + &sym)
                }
            }
            // 'n' => Ok(vec![WState::Terminal(Token:: 'n')]),
            Token::Numbers(_) => Ok(vec![WState::Terminal(cha)]),
            _ => Err("".to_string()),
        },
        WState::Expre => match cha {
            // '+' => Ok(vec![WState::Terminal('+'), WState::Term, WState::Expr]),
            Token::Symbols(sym) => {
                if sym == "+" {
                    Ok(vec![
                        WState::Terminal(Token::Symbols("+".to_string())),
                        WState::Term,
                        WState::Expr,
                    ])
                } else if sym == ")" || sym == "$" {
                    Ok(vec![WState::Empty])
                } else {
                    Err("Expected '+', found ".to_string() + &sym)
                }
            }
            _ => Err("".to_string()),
        },
        WState::Termt => match cha {
            // '*' => Ok(vec![WState::Terminal(), WState::Factor, WState::Termt]),
            Token::Symbols(sym) => {
                if sym == "*" {
                    Ok(vec![
                        WState::Terminal(Token::Symbols("*".to_string())),
                        WState::Factor,
                        WState::Termt,
                    ])
                } else if sym == "+" || sym == ")" || sym == "$" {
                    Ok(vec![WState::Empty])
                } else {
                    Err("Expected '*', found ".to_string() + &sym)
                }
            }
            _ => Err("".to_string()),
        },
        WState::Terminal(_) => Err("expect terminal".to_string()),
        WState::Empty => Err("".to_string()),
        WState::IfStmt => match cha {
            Token::Reserved(sym) => {
                if sym == "if" {
                    Ok(vec![
                        WState::Terminal(Token::Symbols("if".to_string())),
                        WState::Terminal(Token::Symbols("(".to_string())),
                        WState::Expr,
                        WState::Terminal(Token::Symbols(")".to_string())),
                        WState::Terminal(Token::Symbols("{".to_string())),
                        WState::Stmt,
                        WState::Terminal(Token::Symbols("}".to_string())),
                    ])
                } else {
                    Err("Expected 'if', found ".to_string() + &sym)
                }
            }
            _ => Err("".to_string()),
        },

        WState::WhileStmt => match cha {
            Token::Reserved(sym) => {
                if sym == "while" {
                    Ok(vec![
                        WState::Terminal(Token::Symbols("while".to_string())),
                        WState::Terminal(Token::Symbols("(".to_string())),
                        WState::Expr,
                        WState::Terminal(Token::Symbols(")".to_string())),
                        WState::Terminal(Token::Symbols("{".to_string())),
                        WState::Stmt,
                        WState::Terminal(Token::Symbols("}".to_string())),
                    ])
                } else {
                    Err("Expected 'while', found ".to_string() + &sym)
                }
            }
            _ => Err("".to_string()),
        },
        WState::AssignStmt => match cha {
            Token::Reserved(sym) => {
                if sym == "let" {
                    Ok(vec![
                        WState::Terminal(Token::Symbols("let".to_string())),
                        WState::Terminal(Token::Symbols("x".to_string())),
                        WState::Terminal(Token::Symbols("=".to_string())),
                        WState::Expr,
                        WState::Terminal(Token::Symbols(";".to_string())),
                    ])
                } else {
                    Err("Expected 'let', found ".to_string() + &sym)
                }
            }
            _ => Err("".to_string()),
        },
        WState::NewAssignStmt => match cha {
            Token::Reserved(sym) => {
                if sym == "let" {
                    Ok(vec![
                        WState::Terminal(Token::Symbols("let".to_string())),
                        WState::Terminal(Token::Symbols("x".to_string())),
                        WState::Terminal(Token::Symbols("=".to_string())),
                        WState::Expr,
                        WState::Terminal(Token::Symbols(";".to_string())),
                    ])
                } else {
                    Err("Expected 'let', found ".to_string() + &sym)
                }
            }
            _ => Err("".to_string()),
        },
        WState::Stmt => match cha {
            Token::Reserved(sym) => {
                if sym == "if" {
                    Ok(vec![WState::IfStmt, WState::Stmt])
                } else if sym == "while" {
                    Ok(vec![WState::WhileStmt, WState::Stmt])
                } else if sym == "let" {
                    Ok(vec![WState::AssignStmt, WState::Stmt])
                } else {
                    Err("Expected 'if', 'while' or 'let', found ".to_string() + &sym)
                }
            }
            Token::Symbols(sym) => {
                if sym == "}" {
                    Ok(vec![WState::Empty])
                } else {
                    Err("Expected '}', found ".to_string() + &sym)
                }
            }
            _ => Err("".to_string()),
        },
        WState::Prog => match cha {
            Token::Reserved(sym) => {
                if sym == "if" {
                    Ok(vec![WState::IfStmt, WState::Stmt])
                } else if sym == "while" {
                    Ok(vec![WState::WhileStmt, WState::Stmt])
                } else if sym == "let" {
                    Ok(vec![WState::AssignStmt, WState::Stmt])
                } else {
                    Err("Expected 'if', 'while' or 'let', found ".to_string() + &sym)
                }
            }
            Token::Symbols(sym) => {
                if sym == "}" {
                    Ok(vec![WState::Empty])
                } else {
                    Err("Expected '}', found ".to_string() + &sym)
                }
            }
            _ => Err("".to_string()),
        },
    }
}
