use std::process::exit;

use crate::parser::token::Token;

fn check_reserved_word(buffer: &String) -> Option<Token> {
    match buffer.as_str() {
        "func" => Some(Token::Func),
        "int" => Some(Token::Int),
        "float" => Some(Token::Float),
        "bool" => Some(Token::Bool),
        "list" => Some(Token::List),
        "string" => Some(Token::String),
        "let" => Some(Token::Let),
        "return" => Some(Token::Return),
        "if" => Some(Token::If),
        "else" => Some(Token::Else),
        "and" => Some(Token::And),
        "or" => Some(Token::Or),
        "not" => Some(Token::Not),
        "while" => Some(Token::While),
        "for" => Some(Token::For),
        "print" => Some(Token::Print),
        _ => None,
    }
}

fn check_compare_operator(content: &[u8], i: usize) -> Option<Token> {
    let c = content[i];
    if c == b'!' {
        if i + 1 >= content.len() || content[i + 1] != b'=' {
            eprintln!("The ! operator should have a = after it");
            exit(1);
        }
        Some(Token::BangEqual);
    }
    if c == b'=' {
        if i + 1 >= content.len() {
            return Some(Token::Equal);
        } else {
            if content[i + 1] == b'=' {
                return Some(Token::EqualEqual);
            } else {
                return Some(Token::Equal);
            }
        }
    }
    if c == b'>' {
        if i + 1 >= content.len() {
            return Some(Token::Greater);
        } else {
            if content[i + 1] == b'=' {
                return Some(Token::GreaterEqual);
            } else {
                return Some(Token::Greater);
            }
        }
    }
    if c == b'<' {
        if i + 1 >= content.len() {
            return Some(Token::Less);
        } else {
            if content[i + 1] == b'=' {
                return Some(Token::LessEqual);
            } else {
                return Some(Token::Less);
            }
        }
    }
    None
}

fn check_single_char_token(content: &[u8], i: usize) -> Option<Token> {
    match content[i] {
        b' ' => Some(Token::Whitespace),
        b';' => Some(Token::Semicolon),
        b'{' => Some(Token::LeftCurlyBracket),
        b'}' => Some(Token::RightCurlyBracket),
        b'[' => Some(Token::LeftBracket),
        b']' => Some(Token::RightBracket),
        b'(' => Some(Token::LeftParen),
        b')' => Some(Token::RightParen),
        b'+' => Some(Token::Plus),
        b'-' => Some(Token::Minus),
        b'*' => Some(Token::Asterisk),
        b'/' => Some(Token::Slash),
        b'%' => Some(Token::Modulo),
        b',' => Some(Token::Comma),
        b'\t' => Some(Token::Tab),
        b'\n' => Some(Token::Newline),
        _ => None,
    }
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
fn check_bool_literal(buffer: &String) -> bool {
    match buffer.as_str() {
        "True" => true,
        "False" => true,
        _ => false,
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
            i += 1;
            // for now we are ignoring tabs and new lines and blank spaces
            if single_char_token == Token::Newline
                || single_char_token == Token::Tab
                || single_char_token == Token::Whitespace
            {
                continue;
            }
            tokens.push(single_char_token);
            continue;
        }

        // we then check for compare operators
        if let Some(compare_op) = check_compare_operator(&content, i) {
            let increment = match compare_op {
                Token::Less => 1,
                Token::Equal => 1,
                Token::Greater => 1,
                _ => 2,
            };
            tokens.push(compare_op);
            i += increment;
            continue;
        }

        // checking for strings
        if let Some(str) = check_string(&content, i) {
            i += str.len();
            tokens.push(Token::StringLit(str[1..str.len() - 1].to_string()));
            continue;
        }

        // here we found a reserved word or a identifier, and we check it
        if c.is_ascii_alphabetic() || c == b'_' {
            buffer.push(c as char);
            i += 1;
            if i >= content.len() {
                // add token to tokens list and return
                return tokens;
            }
            c = content[i];
            while c.is_ascii_alphanumeric() || c == b'_' {
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
                tokens.push(reserved);
            } else if check_bool_literal(&buffer) {
                tokens.push(Token::BoolLit(match buffer.as_str() {
                    "True" => true,
                    _ => false,
                }));
            } else {
                tokens.push(Token::Identifier(buffer.clone()));
            }
            buffer.clear();
            continue;
        }

        // finnally we check for numbers
        if c.is_ascii_digit() {
            buffer.push(c as char);
            i += 1;
            while i < content.len() && content[i].is_ascii_digit() {
                c = content[i];
                buffer.push(c as char);
                i += 1;
            }
            if i >= content.len() {
                tokens.push(Token::Int64(
                    buffer
                        .parse::<i64>()
                        .expect("Error while parsing int64 during lexing phase"),
                ));
                buffer.clear();
                return tokens;
            }
            c = content[i];
            if c == b'.' {
                buffer.push(c as char);
                i += 1;
                while i < content.len() && content[i].is_ascii_digit() {
                    c = content[i];
                    buffer.push(c as char);
                    i += 1;
                }
                // this is ugly but I think it works
                if i < content.len()
                    && content[i] != b' '
                    && content[i] != b';'
                    && content[i] != b')'
                    && content[i] != b'/'
                    && content[i] != b'+'
                    && content[i] != b'-'
                    && content[i] != b'%'
                    && content[i] != b'*'
                {
                    eprintln!(
                        "Numbers should contains only digits and, in case of floats, a single ."
                    );
                    exit(1);
                } else {
                    tokens.push(Token::Float64(
                        buffer
                            .parse::<f64>()
                            .expect("Error while parsing float64 during lexing phase"),
                    ));
                }
            } else if c == b' '
                || c == b';'
                || c == b')'
                || content[i] != b'/'
                || content[i] != b'+'
                || content[i] != b'-'
                || content[i] != b'%'
                || content[i] != b'*'
            {
                tokens.push(Token::Int64(
                    buffer
                        .parse::<i64>()
                        .expect("Error while parsing int64 during lexing phase"),
                ));
            } else {
                eprintln!("Numbers should contains only digits and, in case of floats, a single .");
                exit(1);
            }
            buffer.clear();
        }
    }
    tokens
}
