use super::table::{DfaState as state, ErrType};

#[warn(clippy::single_match)]
pub fn dfa_transform(current_state: &mut state, current_char: char) {
    match current_state {
        state::Start => match current_char {
            'a'..='z' | 'A'..='Z' | '_' => *current_state = state::LetterNow,
            '0'..='9' => *current_state = state::WholePartNow,
            ';' | '{' | '}' | '(' | ')' | ',' => *current_state = state::SingleSymbolTerminalNow,
            '>' | '=' | '<' | '!' | '+' | '-' | '*' | '/' => {
                *current_state = state::SingleSymbolNow
            }
            '"' => *current_state = state::StringStartNow,
            ' ' | '\n' | '\r' => {}
            _ => *current_state = state::ErrFirst(ErrType::UnexpectedChar),
        },
        state::LetterNow => match current_char {
            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => { /*do nothing */ }
            _ => *current_state = state::Start,
        },
        state::WholePartNow => match current_char {
            '0'..='9' => { /*do nothing */ }
            '.' => *current_state = state::DotNow,
            'a'..='z' | 'A'..='Z' | '_' => *current_state = state::ErrFirst(ErrType::ExpectNumber),
            _ => *current_state = state::Start,
        },
        state::DotNow => match current_char {
            '0'..='9' => *current_state = state::FractionalPart,
            _ => *current_state = state::ErrFirst(ErrType::ExpectNumber),
        },
        state::FractionalPart => match current_char {
            '0'..='9' => { /*do nothing */ }
            'a'..='z' | 'A'..='Z' | '_' => *current_state = state::ErrFirst(ErrType::ExpectNumber),
            _ => *current_state = state::Start,
        },
        state::SingleSymbolNow => match current_char {
            '=' => *current_state = state::DoubleSymbolNow,
            '*' => *current_state = state::CommentNow,
            '/' => *current_state = state::SingleComment,
            _ => *current_state = state::Start,
        },
        state::StringStartNow => match current_char {
            '"' => *current_state = state::StringEndNow,
            '\\' => *current_state = state::StringEscapeNow,
            '\n' | '\r' => *current_state = state::ErrFirst(ErrType::ExpectStringEnd),
            _ => {}
        },
        state::StringEscapeNow => match current_char {
            _ => *current_state = state::StringStartNow,
        },
        state::SingleSymbolTerminalNow => *current_state = state::Start,
        state::StringEndNow => *current_state = state::Start,
        state::ErrFirst(_) => *current_state = state::ErrAlready,
        state::DoubleSymbolNow => *current_state = state::Start,
        state::ErrAlready => {}
        state::CommentNow => match current_char {
            '*' => *current_state = state::CommentEnd,
            _ => {}
        },
        state::CommentEnd => match current_char {
            '/' => *current_state = state::Start,
            _ => *current_state = state::CommentNow,
        },
        state::SingleComment => match current_char {
            '\n' | '\r' => *current_state = state::Start,
            _ => {}
        },
    }
}
