use crate::parser::token::{Reserved, Token, TokenType};

fn check_reserved_word(buffer: &String) -> Option<Reserved> {
    match buffer.as_str() {
        "let" => Some(Reserved::Let),
        _ => None,
    }
}

pub fn tokenize(content: &[u8]) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut buffer = String::from("");
    for mut i in 0..content.len() {
        let mut c = content[i];
        if c.is_ascii_alphabetic() {
            buffer.push(c as char);
            i += 1;
            if i >= content.len() {
                // add token to tokens list and return
                return tokens;
            }
            c = content[i];
            while c.is_ascii_alphanumeric() {
                buffer.push(c as char);
                i += 1;
                if i >= content.len() {
                    // add token to tokens list and return
                    return tokens;
                }
                c = content[i];
            }
            // check reserved word and continue
            if let Some(reserved) = check_reserved_word(&buffer) {
                tokens.push(Token {
                    _type: TokenType::ReservedWord(reserved),
                    value: Some(buffer.clone()),
                });
            } else {
                println!("Not reserved");
            }
        }
        buffer.clear();
    }
    tokens
}
