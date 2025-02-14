use crate::parser::token::Token;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VarType {
    Int64,
    Float64,
    Bool,
    String,
    List,
    Nil,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Parameter {
    pub type_token: Token,
    pub identifier_token: Token,
}

#[derive(Serialize, Deserialize)]
pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

// we are brute forcing the token type for operators here, but this is njot always the case, if
// should fiz this
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Declaration {
    FunctionDeclaration {
        name: Token,
        return_type: Token,
        parameters: Vec<Parameter>,
        body: Vec<Statement>,
    },
    VariableDeclaration {
        name: Token,
        type_: Token,
        initializer_expr: Option<Expr>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    ExprStatement {
        expr: Expr,
    },
    PrintStatement {
        expr: Expr,
    },
    IfStatement {
        expr: Expr,
        stmt: Box<Statement>,
        else_stmt: Option<Box<Statement>>,
    },
    WhileStatement {
        initializer: Option<Box<Statement>>, // Only for when while is used to handle For statements
        expr: Expr,
        stmt: Box<Statement>,
    },
    FunctionStatement {
        name: Token,
        parameters: Vec<Expr>,
    },
    ReturnStatement {
        expr: Option<Expr>,
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

    fn peek(&self) -> Option<&Token> {
        if self.index >= self.tokens.len() {
            return None;
        }
        Some(&self.tokens[self.index])
    }

    fn advance(&mut self) -> &Token {
        let ret = &self.tokens[self.index];
        self.index += 1;
        &ret
    }

    fn previous(&self) -> Token {
        self.tokens[self.index - 1].clone()
    }

    fn consume(&mut self, token: Token, message: &str) -> Token {
        if self.check(&token) {
            return self.advance().clone();
        }
        panic!("{message}")
    }

    fn consume_from_list(&mut self, token_list: &[Token], message: &str) -> Token {
        for token in token_list.iter() {
            if self.check(&token) {
                return self.advance().clone();
            }
        }
        panic!("{message}")
    }

    fn check(&self, token: &Token) -> bool {
        if let Some(token_peeked) = self.peek() {
            return std::mem::discriminant(token) == std::mem::discriminant(token_peeked);
        }
        false
    }

    fn match_up(&mut self, tokens: &[Token]) -> bool {
        for token in tokens.iter() {
            if self.check(&token) {
                self.advance();
                return true;
            }
        }
        false
    }

    pub fn primary(&mut self) -> Expr {
        if self.match_up(&[
            Token::Int64(0),
            Token::Float64(0.0),
            Token::BoolLit(true),
            Token::StringLit("".to_string()),
        ]) {
            return Expr::Litheral {
                value: self.previous(),
            };
        }
        if self.match_up(&[Token::Identifier("".to_string())]) {
            return Expr::Variable {
                value: self.previous(),
            };
        }
        if self.match_up(&[Token::LeftParen]) {
            let expr = self.expression();
            self.consume(Token::RightParen, "we need a closeing right paren");
            return Expr::Grouping {
                middle: Box::new(expr),
            };
        }
        panic!("Expect expression");
    }

    pub fn finish_call(&mut self, callee: Expr) -> Expr {
        let mut arguments: Vec<Expr> = vec![];
        if !self.check(&Token::RightParen) {
            loop {
                arguments.push(self.expression());
                if !self.match_up(&[Token::Comma]) {
                    break;
                }
            }
        }
        let paren = self.consume(Token::RightParen, "expect ')' after arguments");
        Expr::Call {
            callee: Box::new(callee),
            paren: paren.clone(),
            arguments: arguments.clone(),
        }
    }

    pub fn call(&mut self) -> Expr {
        let mut expr = self.primary();
        loop {
            if self.match_up(&[Token::LeftParen]) {
                expr = self.finish_call(expr);
            } else {
                break;
            }
        }
        expr
    }

    pub fn unary(&mut self) -> Expr {
        if self.match_up(&[Token::Not, Token::Minus]) {
            let operator = self.previous();
            let rhs = self.unary();
            return Expr::UnaryExpr {
                op: operator.clone(),
                child: Box::new(rhs),
            };
        }
        self.call()
    }

    pub fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        let asterisk = Token::Asterisk;
        let slash = Token::Slash;
        let modulo = Token::Modulo;
        while self.match_up(&[asterisk.clone(), slash.clone(), modulo.clone()]) {
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
        let minus = Token::Minus;
        let plus = Token::Plus;
        while self.match_up(&[minus.clone(), plus.clone()]) {
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
        let greater = Token::Greater;
        let greater_equal = Token::GreaterEqual;
        let less = Token::Less;
        let less_equal = Token::LessEqual;
        while self.match_up(&[
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

    pub fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        let bang_equal = Token::BangEqual;
        let equal_equal = Token::EqualEqual;
        while self.match_up(&[bang_equal.clone(), equal_equal.clone()]) {
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
        while self.match_up(&[Token::And]) {
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

        while self.match_up(&[Token::Or]) {
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
        if self.match_up(&[Token::Equal]) {
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
        self.consume(Token::Semicolon, "expect ';' after value");
        Statement::PrintStatement { expr }
    }

    pub fn variable_declaration(&mut self) -> Statement {
        let var_type = self.consume_from_list(
            &[Token::Bool, Token::Int, Token::Float, Token::String],
            "expect variable type",
        );
        let name = self.consume(Token::Identifier("".to_string()), "expect variable name.");
        let mut initializer: Option<Expr> = None;
        if self.match_up(&[Token::Equal]) {
            initializer = Some(self.expression());
        }
        self.consume(Token::Semicolon, "expect ';' after var declaration.");
        Statement::Declaration(Declaration::VariableDeclaration {
            name: name.clone(),
            type_: var_type,
            initializer_expr: initializer,
        })
    }

    pub fn function_declaration(&mut self) -> Statement {
        // here we are taking the return type of the function. This is not okay, we should use
        // consume. We need a refactor in token.rs to create specific tokens for type definitions
        let return_type = self.advance().clone();
        let name = self.consume(Token::Identifier("".to_string()), "expect function name");
        self.consume(
            Token::LeftParen,
            "expect '(' after function name and type declaration",
        );
        let mut parameters: Vec<Parameter> = vec![];
        if !self.check(&Token::RightParen) {
            loop {
                let parameter_type = self.consume_from_list(
                    &[Token::Bool, Token::Int, Token::Float, Token::String],
                    "expected parameter type",
                );
                let parameter_name =
                    self.consume(Token::Identifier("".to_string()), "expected parameter name");
                parameters.push(Parameter {
                    type_token: parameter_type,
                    identifier_token: parameter_name,
                });
                if !self.match_up(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen, "expect ')' after parameters");
        self.consume(Token::LeftCurlyBracket, "expect '{' before function body");
        let body = self.block();
        Statement::Declaration(Declaration::FunctionDeclaration {
            name: name.clone(),
            return_type: return_type.clone(),
            parameters: parameters.clone(),
            body: body.clone(),
        })
    }

    pub fn declaration(&mut self) -> Statement {
        if self.match_up(&[Token::Let]) {
            return self.variable_declaration();
        }
        if self.match_up(&[Token::Func]) {
            return self.function_declaration();
        } else {
            return self.statement();
        }
    }

    pub fn expr_statement(&mut self) -> Statement {
        let expr = self.expression();
        self.consume(Token::Semicolon, "expect ';' after value");
        Statement::ExprStatement { expr }
    }

    pub fn block(&mut self) -> Vec<Statement> {
        let mut statements: Vec<Statement> = vec![];
        while !self.check(&Token::RightCurlyBracket) & !self.is_at_end() {
            statements.push(self.declaration());
        }
        self.consume(Token::RightCurlyBracket, "expect '}' after block");
        statements
    }

    pub fn if_statement(&mut self) -> Statement {
        self.consume(Token::LeftParen, "expect '(' after if");
        let condition = self.expression();
        self.consume(Token::RightParen, "expect')' after if condition");
        let then_branch = self.statement();
        let mut else_branch: Option<Statement> = None;
        if self.match_up(&[Token::Else]) {
            else_branch = Some(self.statement());
        }
        Statement::IfStatement {
            expr: condition,
            stmt: Box::new(then_branch),
            else_stmt: match else_branch {
                Some(val) => Some(Box::new(val)),
                None => None,
            },
        }
    }

    pub fn while_statement(&mut self) -> Statement {
        self.consume(Token::LeftParen, "expect '(' after while");
        let condition = self.expression();
        self.consume(Token::RightParen, "expect ')' after condition");
        let body = self.statement();
        Statement::WhileStatement {
            initializer: None,
            expr: condition,
            stmt: Box::new(body),
        }
    }

    pub fn for_statement(&mut self) -> Statement {
        self.consume(Token::LeftParen, "expect '(' after for");
        let mut initializer: Option<Statement> = None;
        if self.match_up(&[Token::Semicolon]) {
            initializer = None;
        } else if self.match_up(&[Token::Let]) {
            initializer = Some(self.variable_declaration());
        } else {
            initializer = Some(self.expr_statement());
        }

        let mut condition: Option<Expr> = None;
        if !self.check(&Token::Semicolon) {
            condition = Some(self.expression());
        }

        self.consume(Token::Semicolon, "expect ';' after loop condition");
        let mut increment: Option<Expr> = None;
        if !self.check(&Token::RightParen) {
            increment = Some(self.expression());
        }
        self.consume(Token::RightParen, "expect ')' after for clauses");
        let mut body = self.statement();
        if let Some(inc) = increment {
            body = Statement::Block {
                statements: vec![body, Statement::ExprStatement { expr: inc }],
            }
        }
        if condition.is_none() {
            condition = Some(Expr::Litheral {
                value: Token::BoolLit(true),
            });
            body = Statement::WhileStatement {
                initializer: match initializer {
                    Some(ini) => Some(Box::new(ini)),
                    _ => None,
                },
                expr: condition.unwrap(),
                stmt: Box::new(body),
            };
        } else {
            body = Statement::WhileStatement {
                initializer: match initializer {
                    Some(ini) => Some(Box::new(ini)),
                    _ => None,
                },
                expr: condition.unwrap(),
                stmt: Box::new(body),
            };
        }
        body
    }

    pub fn return_statement(&mut self) -> Statement {
        let keyword = self.previous();
        let mut value: Option<Expr> = None;
        if !self.check(&Token::Semicolon) {
            value = Some(self.expression());
        }
        self.consume(Token::Semicolon, "expect ';' after return value");
        Statement::ReturnStatement { expr: value }
    }

    pub fn statement(&mut self) -> Statement {
        if self.match_up(&[Token::Print]) {
            return self.print_statement();
        } else if self.match_up(&[Token::LeftCurlyBracket]) {
            return Statement::Block {
                statements: self.block(),
            };
        } else if self.match_up(&[Token::If]) {
            return self.if_statement();
        } else if self.match_up(&[Token::While]) {
            return self.while_statement();
        } else if self.match_up(&[Token::For]) {
            return self.for_statement();
        } else if self.match_up(&[Token::Return]) {
            return self.return_statement();
        } else {
            return self.expr_statement();
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
