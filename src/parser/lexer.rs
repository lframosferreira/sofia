use crate::parser::token::{CompareOp, Reserved, Token, TokenType};

fn check_reserved_word(buffer: &String) -> Option<Reserved> {
    match buffer.as_str() {
        "let" => Some(Reserved::Let),
        "func" => Some(Reserved::Func),
        "int" => Some(Reserved::Int),
        "uint" => Some(Reserved::Uint),
        "float" => Some(Reserved::Float),
        "string" => Some(Reserved::String),
        "return" => Some(Reserved::Return),
        "if" => Some(Reserved::If),
        "else" => Some(Reserved::Else),
        "and" => Some(Reserved::And),
        "or" => Some(Reserved::Or),
        "not" => Some(Reserved::Not),
        "while" => Some(Reserved::While),
        "for" => Some(Reserved::For),
        _ => None,
    }
}

pub fn tokenize(content: &[u8]) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut buffer = String::from("");
    let mut i = 0;
    while i < content.len() {
        let mut c = content[i];
        if c == b' ' {
            tokens.push(Token {
                _type: TokenType::Whitespace,
                value: Some(String::from(c as char)),
            });
            i+=1;
            continue;
        }
        if c == b';' {
            tokens.push(Token {
                _type: TokenType::Semicolon,
                value: Some(String::from(c as char)),
            });
            i+=1;
            continue;
        }
        if c == b'!' {
            tokens.push(Token {
                _type: TokenType::CompareOperator(CompareOp::BangEqual),
                value: Some(String::from(c as char)),
            });
            i+=1;
            continue;
        }
        if c == b'=' {
            tokens.push(Token {
                _type: TokenType::CompareOperator(CompareOp::Equal),
                value: Some(String::from(c as char)),
            });
            i+=1;
            continue;
        }
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
                tokens.push(Token {
                    _type: TokenType::Identifier,
                    value: Some(buffer.clone()),
                });
            }
            buffer.clear();
            continue;
        }
        // por enqt ignora numeros e etc
        i+=1;
    }
    tokens
}
