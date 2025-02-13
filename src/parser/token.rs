use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Token {
    // An integer litheral
    Int64(i64),

    // A float litheral
    Float64(f64),

    // A boolean litheral
    BoolLit(bool),

    // A string litheral
    StringLit(String),

    // The func kw
    Func,

    // The int kw
    Int,

    // The bool kw
    Bool,

    // The float kw
    Float,

    // The string kw
    String,

    // The list kw
    List,

    // The return kw
    Return,

    // The if kw
    If,

    // The else kw
    Else,

    // The let kw
    Let,

    // The and kw
    And,

    // The or kw
    Or,

    // The not kw
    Not,

    // The while kw
    While,

    // The for kw
    For,

    // The print kw
    Print,

    // !=
    BangEqual,

    // =
    Equal,

    // ==
    EqualEqual,

    // >
    Greater,

    // >=
    GreaterEqual,

    // <
    Less,

    // <=
    LessEqual,

    // Asterisk
    // *
    Asterisk,

    // Slash
    // /
    Slash,

    // Modulo operator
    // %
    Modulo,

    // Minus symbol
    // -
    Minus,

    // Plus symbol
    // +
    Plus,

    // Left parenthesis
    // (
    LeftParen,

    // Right parenthesis
    // )
    RightParen,

    // Comma
    // ,
    Comma,

    // Semicolon
    // ;
    Semicolon,

    // An identifier for a variable/function
    // x, sum
    Identifier(String),

    // {
    LeftCurlyBracket,

    // }
    RightCurlyBracket,

    // [
    LeftBracket,

    // ]
    RightBracket,

    // Represents a white space
    Whitespace,

    // Newline
    // \n
    Newline,

    // Tab
    // \t
    Tab,

    // Nil
    Nil,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Eq for Token {}
