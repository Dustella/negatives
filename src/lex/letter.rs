pub fn is_letter(word: char) -> bool {
    (word <= 'Z' && word >= 'A') | (word >= 'a' && word <= 'z') | (word == '_')
}

pub fn is_digit(word: char) -> bool {
    word <= '9' && word >= '0'
}

pub fn la() {}
