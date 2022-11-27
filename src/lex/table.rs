use std::fmt;
const RESERVED: [&str; 10] = [
    "let", "function", "for", "continue", "else", "if", "return", "elif", "while", "break",
];

pub fn is_reversed(para: String) -> bool {
    RESERVED.iter().any(|e| e.to_string() == para)
}

#[derive(Clone)]
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
    ErrFirst(ErrType),
    ErrAlready,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Reserved(String),
    Numbers(String),
    Identifier(String),
    Symbols(String),
    StringLitral(String),
    BooleanLitral(String),
}
#[derive(Clone, Copy)]
pub enum ErrType {
    UnexpectedChar,
    ExpectNumber,
    ExpectStringEnd,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
