#[derive(Debug, PartialEq, Eq)]
pub enum Numeral {
    Int64,
    Float64,
}

// A reserved word in the Sofia language
#[derive(Debug, PartialEq, Eq)]
pub enum Reserved {
    Func,
    Int,
    Float,
    String,
    Return,
    If,
    Else,
    And,
    Or,
    Not,
    While,
    For,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

// A token type in the Sofia language
#[derive(Debug, PartialEq)]
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

    // Minus symbol
    // -
    Minus,

    // Plus symbol
    // +
    Plus,

    // Plus plus symbol
    // ++
    PlusPlus,

    // Semicolon
    // ;
    Semicolon,

    // Asterisk
    // *
    Asterisk,

    // Slash
    // /
    Slash,

    // Modulo operator
    // %
    Modulo,

    // Compare operatoros
    // ==
    CompareOperator(CompareOp),

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
}

#[derive(Debug)]
pub struct Token {
    pub _type: TokenType,
    pub value: Option<String>,
}

impl Token {
    fn new(_type: TokenType, value: Option<String>) -> Token {
        Token { _type, value }
    }
}
