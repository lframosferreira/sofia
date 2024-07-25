use std::process::exit;

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

fn check_compare_operator(content: &[u8], i: usize) -> Option<CompareOp> {
    let c = content[i];
    if c == b'!' {
        if i + 1 >= content.len() || content[i + 1] != b'=' {
            eprintln!("The ! operator should have a = after it");
            exit(1);
        }
        Some(CompareOp::BangEqual);
    }
    if c == b'=' {
        if i + 1 >= content.len() {
            return Some(CompareOp::Equal);
        } else {
            if content[i + 1] == b'=' {
                return Some(CompareOp::EqualEqual);
            } else {
                return Some(CompareOp::Equal);
            }
        }
    }
    if c == b'>' {
        if i + 1 >= content.len() {
            return Some(CompareOp::Greater);
        } else {
            if content[i + 1] == b'=' {
                return Some(CompareOp::GreaterEqual);
            } else {
                return Some(CompareOp::Greater);
            }
        }
    }
    if c == b'<' {
        if i + 1 >= content.len() {
            return Some(CompareOp::Less);
        } else {
            if content[i + 1] == b'=' {
                return Some(CompareOp::LessEqual);
            } else {
                return Some(CompareOp::Less);
            }
        }
    }
    None
}

fn check_single_char_token(content: &[u8], i: usize) -> Option<TokenType> {
    let c = content[i];
    match c {
        b' ' => Some(TokenType::Whitespace),
        b';' => Some(TokenType::Semicolon),
        b'{' => Some(TokenType::LeftCurlyBracket),
        b'}' => Some(TokenType::RightCurlyBracket),
        b'(' => Some(TokenType::LeftParen),
        b')' => Some(TokenType::RightParen),
        b'+' => Some(TokenType::Plus),
        b'-' => Some(TokenType::Minus),
        b'*' => Some(TokenType::Asterisk),
        b'/' => Some(TokenType::Slash),
        b',' => Some(TokenType::Comma),
        b'\n' => Some(TokenType::Newline),
        _ => None,
    }
}

fn check_number(content: &[u8], i: usize) -> Option<TokenType> {
    None
}

fn check_string(content: &[u8], idx: usize) -> Option<String> {
    let starting_i = idx;
    let mut i = idx;
    if content[i] != b'"' {
        return None;
    }
    i += 1;
    while i < content.len() {
        if content[i] == b'"' {
            break;
        }
        i += 1;
    }
    if i >= content.len() {
        eprintln!("String should be delimited by quotes on start and on the end");
        exit(1);
    }
    match String::from_utf8(content[starting_i..i + 1].to_vec()) {
        Ok(str) => Some(str),
        Err(e) => {
            eprintln!(
                "Failed to convert u8 array to string in the check string function: {}",
                e
            );
            None
        }
    }
}

pub fn tokenize(content: &[u8]) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut buffer = String::from("");
    let mut i: usize = 0;
    while i < content.len() {
        let mut c = content[i];

        // first we check for single char tokens and get rid of them as soon as they are seen
        if let Some(single_char_token) = check_single_char_token(&content, i) {
            tokens.push(Token {
                _type: single_char_token,
                value: Some(String::from(c as char)),
            });
            i += 1;
            continue;
        }

        // we then check for compare operators
        if let Some(compare_op) = check_compare_operator(&content, i) {
            let increment = match compare_op {
                CompareOp::Less => 1,
                CompareOp::Equal => 1,
                CompareOp::Greater => 1,
                _ => 2,
            };
            tokens.push(Token {
                _type: TokenType::CompareOperator(compare_op),
                value: Some(String::from(c as char)),
            });
            i += increment;
            continue;
        }

        // checking for strings
        if let Some(str) = check_string(&content, i) {
            i += str.len();
            tokens.push(Token {
                _type: TokenType::String,
                value: Some(str),
            });
            continue;
        }

        // here we found a reserved word or a identifier, and we check it
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

        // finnally we check for numbers
        if let Some(number) = check_number(&content, i) {
            i += 1;
            continue;
        }
    }
    tokens
}
