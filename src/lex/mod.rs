mod inspector;
use self::{gen_token::gen_token, table::ErrType};
pub use crate::lex::inspector::*;
use crate::lex::{dfa::dfa_transform, table::DfaState as state};
use colored::*;
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
        while !(self.current_state.is_start() && !current_char.is_whitespace())
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

            // println!("---------");
            // dbg!(&last_state);
            // dbg!(&buffer);
            // dbg!(&self.current_state);
        }
        let res = gen_token(last_state, buffer);
        Ok(res.unwrap())
    }
}

pub fn tokenize(pre_processed: Vec<String>) -> Vec<Result<Vec<Token>, (usize, String)>> {
    let mut result_tokens = Vec::new();
    for line in pre_processed {
        let mut token_in_this_line: Vec<Token> = Vec::new();
        let mut buffer = String::from("");
        let mut current_state = state::Start;
        let mut err_msg = String::new();
        let mut has_err_this_line = 0;

        let line = line.trim_start();

        for (index, current_char) in line.chars().enumerate() {
            let origin_state = current_state.clone();
            dfa_transform(&mut current_state, current_char);

            if let state::Start = current_state {
                dfa_transform(&mut current_state, current_char);
                let token = gen_token(origin_state, buffer.clone());
                match token {
                    None => {}
                    Some(tok) => token_in_this_line.push(tok),
                }
                buffer.clear();
            }

            if let state::ErrFirst(ii) = current_state {
                has_err_this_line = index + 1;

                err_msg = match ii {
                    table::ErrType::UnexpectedChar => "Unexpected char, found '".to_string(),
                    table::ErrType::ExpectStringEnd => "Expected string end, found '".to_string(),
                    table::ErrType::ExpectNumber => "Expected number, found '".to_string(),
                };
                if current_char == '\n' {
                    err_msg.push_str("\\n");
                } else {
                    err_msg.push(current_char);
                }
                err_msg.push_str("'");
                // break;
            }
            match (&current_state, current_char) {
                (state::StringStartNow, ' ') => buffer.push(' '),
                (_, ' ') => {}
                (_, _) => buffer.push(current_char),
            }
        }
        if has_err_this_line == 0 {
            result_tokens.push(Ok(token_in_this_line));
        } else {
            result_tokens.push(Err((has_err_this_line, err_msg)))
        }
    }
    result_tokens
}

pub fn show_tokens(tokens: &Vec<Result<Vec<Token>, (usize, String)>>, source: Vec<String>) {
    for (index, line) in tokens.iter().enumerate() {
        println!("line{}: {}", index + 1, source[index].trim());
        match line {
            Err((index, err_type)) => {
                println!("Error:{}^", "-".repeat(*index));
                println!(
                    "{} {}",
                    "Err at this line, postion".bright_red(),
                    (*index).to_string().bright_red()
                );
                println!("{}", (*err_type).bright_red());
            }
            _ => {
                for token in line.as_ref().unwrap() {
                    match *token {
                        Token::Boolean(_) => print!("{}, ", token.to_string().green()),
                        Token::Identifier(_) => print!("{}, ", token.to_string().bright_white()),
                        Token::Numbers(_) => print!("{}, ", token.to_string().purple()),
                        Token::Reserved(_) => print!("{}, ", token.to_string().bright_yellow()),
                        Token::Punctuator(_) => print!("{}, ", token.to_string().bright_blue()),
                        Token::Litral(_) => print!("{}, ", token.to_string().purple()),
                    }
                }
            }
        }
        println!();
        println!();
    }
}

mod dfa;
mod gen_token;
mod table;
