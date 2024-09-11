use crate::parser::token::Token;

use super::token::{ArithmeticOp, BinaryOp, CompareOp, Numeral, Reserved, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

// we are brute forcing the token type for operators here, but this is njot always the case, if
// should fiz this
#[derive(Debug, Clone)]
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
    Variable {
        value: Token,
    },
    Logical {
        op: Token,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    // "(" middle ")"
    Grouping {
        middle: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Declaration {
    VariableDeclaration {
        name: Token,
        initializer_expr: Box<Expr>,
    },
    StatementDeclaration {
        name: String,
    },
}

#[derive(Debug, Clone)]
pub enum Statement {
    ExprStatement {
        expr: Box<Expr>,
    },
    PrintStatement {
        expr: Box<Expr>,
    },
    IfStatement {
        expr: Box<Expr>,
        stmt: Box<Statement>,
        else_stmt: Option<Box<Statement>>,
    },
    WhileStatement {
        expr: Box<Expr>,
        stmt: Box<Statement>,
    },
    Declaration(Declaration),
    // 0 or more declarations inside a {}
    Block {
        statements: Vec<Statement>,
    },
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.clone(),
            index: 0,
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        if self.index >= self.tokens.len() {
            return None;
        }
        Some(&self.tokens[self.index])
    }

    pub fn advance(&mut self) -> &Token {
        let ret = &self.tokens[self.index];
        self.index += 1;
        &ret
    }

    pub fn previous(&self) -> Token {
        self.tokens[self.index - 1].clone()
    }

    pub fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        if self.check(&token_type) {
            return self.advance().clone();
        }
        panic!("{message}")
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

    pub fn primary(&mut self) -> Expr {
        if self.match_up(vec![TokenType::Bool]) {
            return Expr::Litheral {
                value: self.peek().unwrap().clone(),
            };
        }
        if self.match_up(vec![
            TokenType::Number(Numeral::Int64),
            TokenType::Number(Numeral::Float64),
            TokenType::String,
        ]) {
            return Expr::Litheral {
                value: self.previous(),
            };
        }
        if self.match_up(vec![TokenType::Identifier]) {
            return Expr::Variable {
                value: self.previous(),
            };
        }
        if self.match_up(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "we need a closeing right paren");
            return Expr::Grouping {
                middle: Box::new(expr),
            };
        }
        panic!("Expect expression");
    }

    pub fn unary(&mut self) -> Expr {
        if self.match_up(vec![
            TokenType::ReservedWord(Reserved::Not),
            TokenType::BinaryOperator(BinaryOp::ArithmeticOperator(ArithmeticOp::Minus)),
        ]) {
            let operator = self.previous();
            let rhs = self.unary();
            return Expr::UnaryExpr {
                op: operator.clone(),
                child: Box::new(rhs),
            };
        }
        self.primary()
    }

    pub fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        let asterisk =
            TokenType::BinaryOperator(BinaryOp::ArithmeticOperator(ArithmeticOp::Asterisk));
        let slash = TokenType::BinaryOperator(BinaryOp::ArithmeticOperator(ArithmeticOp::Slash));
        let modulo = TokenType::BinaryOperator(BinaryOp::ArithmeticOperator(ArithmeticOp::Modulo));
        while self.match_up(vec![asterisk.clone(), slash.clone(), modulo.clone()]) {
            let operator = self.previous();
            let rhs = self.unary();
            expr = Expr::BinaryExpr {
                op: operator.clone(),
                lhs: Box::new(expr),
                rhs: Box::new(rhs),
            }
        }
        expr
    }

    pub fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        let minus = TokenType::BinaryOperator(BinaryOp::ArithmeticOperator(ArithmeticOp::Minus));
        let plus = TokenType::BinaryOperator(BinaryOp::ArithmeticOperator(ArithmeticOp::Plus));
        while self.match_up(vec![minus.clone(), plus.clone()]) {
            let operator = self.previous();
            let rhs = self.factor();
            expr = Expr::BinaryExpr {
                op: operator.clone(),
                lhs: Box::new(expr),
                rhs: Box::new(rhs),
            }
        }
        expr
    }

    pub fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        let greater = TokenType::BinaryOperator(BinaryOp::CompareOperator(CompareOp::Greater));
        let greater_equal =
            TokenType::BinaryOperator(BinaryOp::CompareOperator(CompareOp::GreaterEqual));
        let less = TokenType::BinaryOperator(BinaryOp::CompareOperator(CompareOp::GreaterEqual));
        let less_equal =
            TokenType::BinaryOperator(BinaryOp::CompareOperator(CompareOp::GreaterEqual));
        while self.match_up(vec![
            greater.clone(),
            greater_equal.clone(),
            less.clone(),
            less_equal.clone(),
        ]) {
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

    pub fn and(&mut self) -> Expr {
        let mut expr = self.equality();
        while self.match_up(vec![TokenType::ReservedWord(Reserved::And)]) {
            let operator = self.previous();
            let rhs = self.equality();
            expr = Expr::Logical {
                op: operator.clone(),
                lhs: Box::new(expr),
                rhs: Box::new(rhs),
            }
        }
        expr
    }

    pub fn or(&mut self) -> Expr {
        let mut expr = self.and();

        while self.match_up(vec![TokenType::ReservedWord(Reserved::Or)]) {
            let operator = self.previous();
            let rhs = self.and();
            expr = Expr::Logical {
                op: operator.clone(),
                lhs: Box::new(expr),
                rhs: Box::new(rhs),
            }
        }
        expr
    }

    pub fn assignment(&mut self) -> Expr {
        let expr = self.or();
        if self.match_up(vec![TokenType::BinaryOperator(BinaryOp::CompareOperator(
            CompareOp::Equal,
        ))]) {
            let equals = self.previous();
            let assig_value = self.assignment();

            if let Expr::Variable { value } = expr {
                return Expr::Assign {
                    name: value.clone(),
                    value: Box::new(assig_value),
                };
            }
        }
        expr
    }

    pub fn expression(&mut self) -> Expr {
        self.assignment()
    }

    pub fn print_statement(&mut self) -> Statement {
        let expr = self.expression(); // here we are accepting any kind of expression but it should
                                      // be only groupings
        self.consume(TokenType::Semicolon, "expect ';' after value");
        Statement::PrintStatement {
            expr: Box::new(expr),
        }
    }

    pub fn expr_statement(&mut self) -> Statement {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "expect ';' after value");
        Statement::ExprStatement {
            expr: Box::new(expr),
        }
    }

    pub fn block(&mut self) -> Vec<Statement> {
        let mut statements: Vec<Statement> = vec![];
        while !self.check(&TokenType::RightCurlyBracket) & !self.is_at_end() {
            statements.push(self.declaration());
        }
        self.consume(TokenType::RightCurlyBracket, "expect '}' after block");
        statements
    }

    pub fn if_statement(&mut self) -> Statement {
        self.consume(TokenType::LeftParen, "expect '(' after if");
        let condition = self.expression();
        self.consume(TokenType::RightParen, "expect')' after if condition");
        let then_branch = self.statement();
        let mut else_branch: Option<Statement> = None;
        if self.match_up(vec![TokenType::ReservedWord(Reserved::Else)]) {
            else_branch = Some(self.statement());
        }
        Statement::IfStatement {
            expr: Box::new(condition),
            stmt: Box::new(then_branch),
            else_stmt: match else_branch {
                Some(val) => Some(Box::new(val)),
                None => None,
            },
        }
    }

    pub fn statement(&mut self) -> Statement {
        if self.match_up(vec![TokenType::ReservedWord(Reserved::Print)]) {
            return self.print_statement();
        } else if self.match_up(vec![TokenType::LeftCurlyBracket]) {
            return Statement::Block {
                statements: self.block(),
            };
        } else if self.match_up(vec![TokenType::ReservedWord(Reserved::If)]) {
            return self.if_statement();
        } else {
            return self.expr_statement();
        }
    }

    pub fn variable_declaration(&mut self) -> Statement {
        let name = self.consume(TokenType::Identifier, "expect variable name.");
        let mut initializer: Option<Expr> = None;
        if self.match_up(vec![TokenType::BinaryOperator(BinaryOp::CompareOperator(
            CompareOp::Equal,
        ))]) {
            initializer = Some(self.expression());
        }
        self.consume(TokenType::Semicolon, "expect ';' after var declaration.");
        Statement::Declaration(Declaration::VariableDeclaration {
            name: name.clone(),
            initializer_expr: Box::new(initializer.unwrap()),
        })
    }

    pub fn declaration(&mut self) -> Statement {
        if self.match_up(vec![TokenType::ReservedWord(Reserved::Let)]) {
            return self.variable_declaration();
        } else {
            return self.statement();
        }
    }

    pub fn is_at_end(&self) -> bool {
        if let Some(_) = self.peek() {
            return false;
        } else {
            return true;
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements: Vec<Statement> = vec![];
        while !self.is_at_end() {
            statements.push(self.declaration());
        }
        statements
    }
}
