use super::table::{is_reversed, DfaState as state, Token};

pub fn gen_token(origin_state: state, buffer: String) -> Option<Token> {
    let buffer = buffer.trim().to_string();
    match origin_state {
        state::FractionalPart => Some(Token::Constant(buffer.clone())),
        state::WholePartNow => Some(Token::Constant(buffer.clone())),
        state::LetterNow => {
            // check if is in reserved
            if is_reversed(buffer.clone()) {
                Some(Token::Reserved(buffer.clone()))
            } else {
                Some(Token::Identifier(buffer.clone()))
            }
        }
        state::DoubleSymbolNow | state::SingleSymbolNow | state::SingleSymbolTerminalNow => {
            Some(Token::Operator(buffer.clone()))
        }
        state::StringEndNow => Some(Token::StringLiteral(
            buffer.clone().trim_matches('\"').to_string(),
        )),
        _ => None,
    }
}
