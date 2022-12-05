use super::super::lex::Token;
use super::states::WState;

pub fn trans(state: &WState, cha: Token) -> Result<Vec<WState>, String> {
    match state {
        WState::Expr => match cha {
            Token::Operator(sym) => {
                if sym == "(" {
                    Ok(vec![WState::Term, WState::Expr])
                } else if sym == ";" || sym == ")" {
                    Ok(vec![WState::Empty])
                } else {
                    Err("error".to_string())
                }
            }
            Token::Constant(_) | Token::Identifier(_) => Ok(vec![WState::Term, WState::Expre]),
            _ => Err("".to_string()),
        },
        WState::Term => match cha {
            // '(' => Ok(vec![WState::Factor, WState::Termt]),
            Token::Operator(sym) => {
                if sym == "(" {
                    Ok(vec![WState::Factor, WState::Termt])
                } else {
                    Err("Expected '(', found ".to_string() + &sym)
                }
            }
            // 'n' => Ok(vec![WState::Factor, WState::Termt]),
            Token::Constant(_) | Token::Identifier(_) => Ok(vec![WState::Factor, WState::Termt]),
            _ => Err("".to_string()),
        },
        WState::Factor => match cha {
            // Token::Operator(String::from('(')) => Ok(vec![
            //     WState::Terminal(Token::Operator("(".to_string())),
            //     WState::Expr,
            //     WState::Terminal(Token::Operator(")".to_string())),
            // ]),
            Token::Operator(sym) => {
                if sym == "(" {
                    Ok(vec![
                        WState::Terminal(Token::Operator("(".to_string())),
                        WState::Expr,
                        WState::Terminal(Token::Operator(")".to_string())),
                    ])
                } else {
                    Err("Expected '(', found ".to_string() + &sym)
                }
            }
            // 'n' => Ok(vec![WState::Terminal(Token:: 'n')]),
            Token::Constant(_) | Token::Identifier(_) => Ok(vec![WState::Terminal(cha)]),
            _ => Err("".to_string()),
        },
        WState::Expre => match cha {
            // '+' => Ok(vec![WState::Terminal('+'), WState::Term, WState::Expr]),
            Token::Operator(sym) => {
                if sym == "+" {
                    Ok(vec![
                        WState::Terminal(Token::Operator("+".to_string())),
                        WState::Term,
                        WState::Expr,
                    ])
                } else if sym == "-" {
                    Ok(vec![
                        WState::Terminal(Token::Operator("-".to_string())),
                        WState::Term,
                        WState::Expr,
                    ])
                } else if sym == ")" || sym == "$" || sym == ";" {
                    Ok(vec![WState::Empty])
                } else {
                    Err("Expected '+', found ".to_string() + &sym)
                }
            }
            _ => Err("".to_string()),
        },
        WState::Termt => match cha {
            // '*' => Ok(vec![WState::Terminal(), WState::Factor, WState::Termt]),
            Token::Operator(sym) => {
                if sym == "*" {
                    Ok(vec![
                        WState::Terminal(Token::Operator("*".to_string())),
                        WState::Factor,
                        WState::Termt,
                    ])
                } else if sym == "/" {
                    Ok(vec![
                        WState::Terminal(Token::Operator("/".to_string())),
                        WState::Factor,
                        WState::Termt,
                    ])
                } else if sym == "==" {
                    Ok(vec![
                        WState::Terminal(Token::Operator("==".to_string())),
                        WState::Factor,
                        WState::Termt,
                    ])
                } else if sym == "+" || sym == ")" || sym == "$" || sym == ";" {
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
                        WState::Terminal(Token::Reserved("if".to_string())),
                        WState::Terminal(Token::Operator("(".to_string())),
                        WState::Expr,
                        WState::Terminal(Token::Operator(")".to_string())),
                        WState::Terminal(Token::Operator("{".to_string())),
                        WState::Stmt,
                        WState::Terminal(Token::Operator("}".to_string())),
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
                        WState::Terminal(Token::Reserved("while".to_string())),
                        WState::Terminal(Token::Operator("(".to_string())),
                        WState::Expr,
                        WState::Terminal(Token::Operator(")".to_string())),
                        WState::Terminal(Token::Operator("{".to_string())),
                        WState::Stmt,
                        WState::Terminal(Token::Operator("}".to_string())),
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
                        WState::Terminal(Token::Reserved("let".to_string())),
                        WState::Terminal(Token::Identifier("_".to_string())),
                        WState::Terminal(Token::Operator("=".to_string())),
                        WState::Expr,
                        WState::Terminal(Token::Operator(";".to_string())),
                    ])
                } else {
                    Err("Expected 'let', found ".to_string() + &sym)
                }
            }
            _ => Err("".to_string()),
        },
        WState::NewAssignStmt => Ok(vec![
            WState::Terminal(Token::Identifier("x".to_string())),
            WState::Terminal(Token::Operator("=".to_string())),
            WState::Expr,
            WState::Terminal(Token::Operator(";".to_string())),
        ]),
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
            Token::Operator(sym) => {
                if sym == "}" || sym == "$" {
                    Ok(vec![WState::Empty])
                } else {
                    Err("Expected '}', found ".to_string() + &sym)
                }
            }
            Token::Identifier(_) => Ok(vec![WState::NewAssignStmt, WState::Stmt]),
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
            Token::Operator(sym) => {
                if sym == "}" || sym == "$" {
                    Ok(vec![WState::Empty])
                } else {
                    Err("Expected '}', found ".to_string() + &sym)
                }
            }
            _ => Err("".to_string()),
        },
    }
}
