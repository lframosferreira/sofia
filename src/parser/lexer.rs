use crate::parser::token::{Token, TokenType};

pub fn tokenize(content: String) -> Vec<Token> {
    let tokens: Vec<Token> = vec![];
    let mut idx = 0;
    while let Some(c) = content.chars().nth(idx) {
        idx+=1;
    }
    tokens
}
