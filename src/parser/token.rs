#[derive(Debug, PartialEq, Eq)]
pub enum Integer {
    Int64(i64),
    UnsignedInt64(u64),
}

#[derive(Debug, PartialEq)]
pub enum Rational {
    Float64(f64),
}

// A reserved word in the Sofia language
#[derive(Debug, PartialEq, Eq)]
pub enum Reserved {
    Let,
    Func,
    Int,
    Uint,
    Float,
    String,
    Return,
    If,
    Else,
    And,
    Or,
    While,
    For,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CompareOperator {
    // !=
    BangEqual,
    // =
    Equal,
    // ==
    EqualEqual,
    // >
    Greter,
    // >=
    GreaterEqual,
    // <
    Less,
    // <=
    LessEqual,
}

// A token type in the Sofia language
#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Open parenthesis
    // (
    OpenParen,

    // Close parenthesis
    // )
    CloseParen,

    // Comma
    // ,
    Comma,

    // Dot
    // .
    Dot,

    // Minus symbol
    // -
    Minus,

    // Plus symbol
    // +
    Plus,

    // Semicolon
    // ;
    Semicolon,

    // Asterisk
    // *
    Asterisk,

    // Slash
    // /
    Slash,

    // Compare operatoros
    // ==
    CompareOperator,

    // An identifier for a variable/function
    // x, sum
    Identifier,

    // An integer numeral literal
    // 42
    Numeral(Integer),

    // A decimal numeral literal
    // 8.1853527
    Decimal(Rational),

    // A boolean literal
    // True, False
    Bool(bool),

    // A string literal
    // "Hello world"
    String(String),

    // {
    LeftCurlyBracket,

    // }
    RightCurlyBracket,

    // A reversed word
    ReservedWord(Reserved),

    // Represents end of input
    Eof,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenType,
    lexeme: String,
}

impl Token {
    fn new(kind: TokenType, lexeme: String) -> Token {
        Token { kind, lexeme }
    }
}
