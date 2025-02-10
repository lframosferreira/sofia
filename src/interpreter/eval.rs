use crate::parser::parser::{Expr, Statement};
use crate::parser::token::{ArithmeticOp, Numeral, Token, TokenType};

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

// vec of tuples which, in this case, are identifiers only
pub struct Environment {
    variables: Vec<Variable>,
}

impl Environment {
    pub fn new() -> Self {
        return Environment { variables: vec![] };
    }

    pub fn find(&self, name: String) -> Variable {
        if let Some(element) = self.variables.iter().rfind(|&x| x.name == name) {
            dbg!("found var");
            return element.clone();
        } else {
            panic!("not declared variable");
        }
    }
}

fn eval_expr(expr: &Expr) -> Option<Value> {
    match expr {
        Expr::Grouping { middle } => eval_expr(middle),
        Expr::Litheral { value } => {
            match value._type {
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
            }
        }
        _ => None,
    }
}

pub fn eval(code: Vec<Statement>) {
    for stmt in code.iter() {
        match stmt {
            Statement::PrintStatement { expr } => {
                let expr_val = eval_expr(expr);
                if let Some(expr_val) = eval_expr(expr) {
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
            _ => println!("Not implemented yet for eval"),
        }
    }
}
