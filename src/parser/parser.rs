use crate::parser::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

pub enum Expr {
    T,
}

pub enum Node {
    Litheral { value: Token },
    UnaryExpr { child: Box<Node> },
    BinaryExpr { lhs: Box<Node>, rhs: Box<Node> },
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            index: 0,
        }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        if self.index >= self.tokens.len() {
            return None;
        }
        Some(&self.tokens[self.index])
    }

    pub fn consume(&mut self) -> &Token {
        let ret = &self.tokens[self.index];
        self.index += 1;
        &ret
    }

    pub fn parse(&self) -> Option<u32> {
        None
    }
}
