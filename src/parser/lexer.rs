


#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    OpenParen,
    CloseParen,
    Symbol(string),
    Keyword(string),
    Numeral(Integer),
    Decimal(Rational),
    String(String),
    LeftCurlyBracket,
    RightCurlyBracket,
    ReservedWord(Reserved)
}

