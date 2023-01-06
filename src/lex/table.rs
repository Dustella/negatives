use std::fmt;
const KEYWORD: [&str; 10] = [
    "let", "function", "for", "continue", "else", "if", "return", "elif", "while", "break",
];

pub fn is_reversed(para: String) -> bool {
    KEYWORD.iter().any(|e| e.to_string() == para)
}

#[derive(Clone, Debug)]
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
    StringEscapeNow,
    StringEndNow,
    ErrFirst(ErrType),
    ErrAlready,
    CommentNow,
    CommentEnd,
    SingleComment,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Keyword(String),
    Constant(String),
    Identifier(String),
    Operator(String),
    StringLiteral(String),
}
#[derive(Clone, Copy, Debug)]
pub enum ErrType {
    UnexpectedChar,
    ExpectNumber,
    ExpectStringEnd,
    Null, // for parser errs
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}, ", self)
    }
}

impl fmt::Display for ErrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}, found", self)
    }
}

impl DfaState {
    pub fn is_start(&self) -> bool {
        match self {
            DfaState::Start => true,
            _ => false,
        }
    }
    pub fn is_err_first(&self) -> bool {
        match self {
            DfaState::ErrFirst(_) => true,
            _ => false,
        }
    }
    pub fn is_comment(&self) -> bool {
        match &self {
            DfaState::CommentNow => true,
            DfaState::CommentEnd => true,
            DfaState::SingleComment => true,
            _ => false,
        }
    }
}
