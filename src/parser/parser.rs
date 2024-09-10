use crate::parser::token::Token;

use super::token::{BinaryOp, CompareOp, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

// we are brute forcing the token type for operators here, but this is njot always the case, if
// should fiz this
#[derive(Debug)]
pub enum Expr {
    Litheral {
        value: Token,
    },
    UnaryExpr {
        op: Token,
        child: Box<Expr>,
    },
    BinaryExpr {
        op: Token,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            index: 0,
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        if self.index >= self.tokens.len() {
            return None;
        }
        Some(&self.tokens[self.index])
    }

    // AKA consume
    pub fn advance(&mut self) -> &Token {
        let ret = &self.tokens[self.index];
        self.index += 1;
        &ret
    }

    pub fn previous(&self) -> &Token {
        &self.tokens[self.index - 1]
    }

    pub fn check(&self, token_type: &TokenType) -> bool {
        if let Some(token) = self.peek() {
            return token._type == *token_type;
        }
        false
    }

    pub fn match_up(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(&token_type) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    pub fn term(&self) -> Expr {}

    pub fn comparison(&self) -> Expr {
        let mut expr = self.term();
        while self.match_up(vec![]) {
            let operator = self.previous();
            let rhs = self.term();
            expr = Expr::BinaryExpr {
                op: operator.clone(),
                lhs: Box::new(expr),
                rhs: Box::new(rhs),
            }
        }
        expr
    }

    // I should porbably use a macro here in the vec! parameter
    pub fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        let bang_equal = TokenType::BinaryOperator(BinaryOp::CompareOperator(CompareOp::BangEqual));
        let equal_equal =
            TokenType::BinaryOperator(BinaryOp::CompareOperator(CompareOp::EqualEqual));
        while self.match_up(vec![bang_equal.clone(), equal_equal.clone()]) {
            let operator = self.previous();
            let rhs = self.comparison();
            expr = Expr::BinaryExpr {
                op: operator.clone(),
                lhs: Box::new(expr),
                rhs: Box::new(rhs),
            };
        }
        expr
    }

    pub fn expression(&mut self) -> Expr {
        self.equality()
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }
}
