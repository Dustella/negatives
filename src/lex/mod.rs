mod inspector;
use self::{gen_token::gen_token, table::ErrType};
pub use crate::lex::inspector::*;
use crate::lex::{dfa::dfa_transform, table::DfaState as state};
// expose state to public
pub use crate::lex::table::Token;

pub struct Tokenizer {
    current_state: state,
    current_line: usize,
    current_location_inline: usize,
    index: usize,
    source: String,
    pub finished: bool,
}

impl Tokenizer {
    pub fn new(source: String) -> Self {
        Self {
            current_state: state::Start,
            current_line: 0,
            current_location_inline: 0,
            index: 0,
            source,
            finished: false,
        }
    }
    pub fn get_current_line(&self) -> String {
        self.source
            .lines()
            .nth(self.current_line)
            .unwrap()
            .to_string()
    }

    pub fn get_last_char(&self) -> char {
        self.source.chars().nth(self.index - 1).unwrap()
    }
    fn move_next(&mut self) {
        self.index += 1;
        self.current_location_inline += 1;
        if let Some('\n') = self.source.chars().nth(self.index - 1) {
            self.current_line += 1;
            self.current_location_inline = 0;
        }
    }

    pub fn get_next_token(&mut self) -> Result<Token, (String, usize, ErrType)> {
        let mut buffer = String::new();
        let mut last_state = state::Start;
        let maybe_current_char = self.source.chars().nth(self.index);
        //  if current char is none, then it is the end of the file
        if maybe_current_char.is_none() {
            self.finished = true;
            return Ok(Token::Punctuator("$".to_string()));
        }
        let mut current_char = maybe_current_char.unwrap();
        while current_char.is_whitespace() {
            self.move_next();
            let maybe_current_char = self.source.chars().nth(self.index);
            if maybe_current_char.is_none() {
                self.finished = true;
                return Ok(Token::Punctuator("$".to_string()));
            }
            current_char = maybe_current_char.unwrap();
        }
        dfa_transform(&mut self.current_state, current_char);
        buffer.push(current_char);
        while !(self.current_state.is_start() && !last_state.is_comment() && !last_state.is_start())
            && !self.current_state.is_err_first()
        {
            last_state = self.current_state.clone();
            self.move_next();
            let maybe_current_char = self.source.chars().nth(self.index);
            //  if current char is none, then it is the end of the file
            if maybe_current_char.is_none() {
                self.finished = true;
                break;
            }
            let current_char = maybe_current_char.unwrap();
            dfa_transform(&mut self.current_state, current_char);
            if !self.current_state.is_start() {
                // self.index -= 1;
                buffer.push(current_char);
            }
            if let state::ErrFirst(err_type) = self.current_state {
                self.current_state = state::Start;
                let err = Err((
                    self.get_current_line(),
                    self.current_location_inline - 1,
                    err_type,
                ));
                self.move_next();
                return err;
            }
            if self.current_state.is_comment() {
                buffer.clear();
            } // println!("{:?}:{:?} {}", last_state, self.current_state, buffer);
        }
        // debug

        let res = gen_token(last_state, buffer);
        Ok(res.unwrap())
    }
}

mod dfa;
mod gen_token;
mod table;
