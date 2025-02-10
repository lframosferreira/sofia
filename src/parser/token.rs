use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Numeral {
    Int64,
    Float64,
}

// A reserved word in the Sofia language
#[derive(Debug, PartialEq, Clone, Eq, Serialize, Deserialize)]
pub enum Reserved {
    Func,
    Int,
    Bool,
    Float,
    String,
    Return,
    If,
    Else,
    Let,
    And,
    Or,
    Not,
    While,
    For,
    Print,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompareOp {
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArithmeticOp {
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOp {
    CompareOperator(CompareOp),
    ArithmeticOperator(ArithmeticOp),
}

// A token type in the Sofia language
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenType {
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

    // A binary operator, such as arithmetic operators like '+' and compare operators like '='
    BinaryOperator(BinaryOp),

    // An identifier for a variable/function
    // x, sum
    Identifier,

    // An integer numeral literal
    // 42
    Number(Numeral),

    // A boolean literal
    // True, False
    Bool,

    // A string literal
    // "Hello world"
    String,

    // {
    LeftCurlyBracket,

    // }
    RightCurlyBracket,

    // A reversed word
    ReservedWord(Reserved),

    // Represents a white space
    Whitespace,

    // Newline
    // \n
    Newline,

    // Tab
    // \t
    Tab,

    // Arrow
    // ->
    Arrow,

    // Nil
    Nil,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub _type: TokenType,
    pub value: Option<String>,
}

impl Token {
    pub fn new(_type: TokenType, value: Option<String>) -> Token {
        Token { _type, value }
    }
}
