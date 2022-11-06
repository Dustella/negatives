use std::fmt;
const RESERVED: [&str; 10] = [
    "var", "function", "for", "continue", "else", "if", "return", "elif", "while", "break",
];

pub fn is_reversed(para: String) -> bool {
    RESERVED.iter().any(|e| e.to_string() == para)
}
// const operator_table: [&str; 27] = [
//     "+", "-", "*", "/", "<", "<=", ">", ">=", "=", "==", "!=", ",", "(", ")", "^", ",", "\"", "\'",
//     "[", "]", "{", "}", "\\", ".", "?", ":", "!",
// ];

#[derive(Copy, Clone)]
pub enum DfaState {
    Start,
    LetterNow,
    WholePartNow,
    FractionalPart,
    DotNow,
    SingleSymbolTerminalNow,
    SingleSymbolNow,
    DoubleSymbolNow,
    StringStartNow,
    StringEndNow,
    ErrFirst,
    ErrAlready,
}
#[derive(Clone, Debug)]
pub enum Token {
    Reserved(String),
    Numbers(String),
    Identifier(String),
    Symbols(String),
    StringLitral(String),
    BooleanLitral(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
