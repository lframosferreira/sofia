use crate::parser::parser::{Declaration, Expr, Statement};
use crate::parser::token::{ArithmeticOp, BinaryOp, Numeral, Token, TokenType};

#[derive(Debug, Clone)]
enum Value {
    Int64(i64),
    UInt64(u64),
    Float64(f64),
    Bool(bool),
    Str(String),
}

impl Value {
    fn to_string(&self) -> String {
        match self {
            Value::Int64(i) => i.to_string(),
            Value::UInt64(u) => u.to_string(),
            Value::Float64(f) => f.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Str(s) => s.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct Variable {
    name: String,
    value: Value,
}

// vec of variables which, in this case, are identifiers only
struct Environment {
    variables: Vec<Variable>,
}

impl Environment {
    pub fn new() -> Self {
        return Environment { variables: vec![] };
    }

    pub fn insert(&mut self, name: &String, value: &Value) {
        self.variables.push(Variable {
            name: name.clone(),
            value: value.clone(),
        });
    }

    pub fn find(&self, name: &String) -> &Variable {
        if let Some(variable) = self.variables.iter().rfind(|&x| x.name == *name) {
            return variable;
        } else {
            panic!("Varibale {:?} not declared", name);
        }
    }

    fn find_mut(&mut self, name: &String) -> &mut Variable {
        if let Some(variable) = self.variables.iter_mut().rfind(|x| x.name == *name) {
            return variable;
        } else {
            panic!("Varibale {:?} not declared", name);
        }
    }

    pub fn change(&mut self, name: &String, value: &Value) {
        let mut variable = self.find_mut(name);
        println!("{:?}", variable);
        variable.value = value.clone();
    }
}

pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        return Interpreter {
            env: Environment::new(),
        };
    }

    fn eval_expr(&mut self, expr: &Expr) -> Option<Value> {
        match expr {
            Expr::Grouping { middle } => self.eval_expr(middle),
            Expr::Litheral { value } => match value._type {
                TokenType::String => Some(Value::Str(value.value.clone().unwrap())),
                TokenType::Number(Numeral::Int64) => Some(Value::Int64(
                    value.value.clone().unwrap().parse::<i64>().expect(
                        "Error parsing integer 64 from string while evaluating integer litheral",
                    ),
                )),
                TokenType::Number(Numeral::Float64) => Some(Value::Float64(
                    value.value.clone().unwrap().parse::<f64>().expect(
                        "Error parsing float 64 from string while evaluating integer litheral",
                    ),
                )),
                TokenType::Bool => Some(Value::Bool(if value.value.clone().unwrap() == "True" {
                    true
                } else {
                    false
                })),
                _ => None,
            },
            Expr::BinaryExpr { op, lhs, rhs } => {
                let lhs_val = self.eval_expr(lhs).unwrap();
                let rhs_val = self.eval_expr(rhs).unwrap();
                match op._type {
                    TokenType::BinaryOperator(BinaryOp::ArithmeticOperator(ArithmeticOp::Plus)) => {
                        use Value::*;
                        match (lhs_val.clone(), rhs_val.clone()) {
                            (Int64(l), Int64(r)) => Some(Int64(l + r)),
                            (UInt64(l), UInt64(r)) => Some(UInt64(l + r)),
                            (Float64(l), Float64(r)) => Some(Float64(l + r)),
                            (Str(l), Str(r)) => Some(Str(format!("{}{}", l, r))),
                            _ => panic!("Can't sum value variant {:?} with {:?}", lhs_val, rhs_val),
                        }
                    }
                    _ => None,
                }
            }
            Expr::Variable { value } => {
                let identifier = value.value.clone().unwrap();
                let variable = self.env.find(&identifier);
                Some(variable.value.clone())
            }
            Expr::Assign { name, value } => {
                if let Some(expr_value) = self.eval_expr(value) {
                    self.env.change(&name.value.clone().unwrap(), &expr_value);
                    Some(expr_value)
                } else {
                    panic!("Expression in aiignment doesn't evaluate to anything, should be NIL in future");
                }
            }
            _ => None,
        }
    }

    pub fn eval_decl(&mut self, declaration: &Declaration) -> Option<Value> {
        match declaration {
            Declaration::VariableDeclaration {
                name,
                initializer_expr,
            } => {
                if let Some(expr_value) = self.eval_expr(initializer_expr) {
                    self.env.insert(&name.value.clone().unwrap(), &expr_value);
                    Some(expr_value)
                } else {
                    panic!("Expression in aiignment doesn't evaluate to anything, should be NIL in future");
                }
            }
            _ => {
                panic!("Not implemented for this declaration yet")
            }
        }
    }

    fn eval_stmt(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Block { statements } => {
                for stmt in statements.iter() {
                    self.eval_stmt(stmt);
                }
            }
            Statement::PrintStatement { expr } => {
                if let Some(expr_val) = self.eval_expr(expr) {
                    match expr_val {
                        Value::Int64(i) => println!("{:?}", i),
                        Value::UInt64(u) => println!("{:?}", u),
                        Value::Float64(f) => println!("{:?}", f),
                        Value::Bool(b) => println!("{:?}", b),
                        Value::Str(s) => println!("{:?}", s),
                    }
                } else {
                    println!();
                }
            }
            Statement::ExprStatement { expr } => _ = self.eval_expr(expr),
            Statement::IfStatement {
                expr,
                stmt,
                else_stmt,
            } => {
                if let Some(value) = self.eval_expr(expr) {
                    if let Value::Bool(b) = value {
                        if (b) {
                            self.eval_stmt(stmt);
                        } else {
                            if let Some(else_stmt_unwrapped) = else_stmt {
                                self.eval_stmt(else_stmt_unwrapped);
                            }
                        }
                    } else {
                        panic!("If expression should evaluate to boolean");
                    }
                } else {
                    panic!("If expression doesn't evaluate to anything");
                }
            }
            Statement::Declaration(declaration) => {
                self.eval_decl(declaration);
            }
            _ => println!("Not implemented yet for eval: {:?}", stmt),
        }
    }

    pub fn eval(&mut self, code: Vec<Statement>) {
        for stmt in code.iter() {
            self.eval_stmt(stmt);
        }
    }
}
