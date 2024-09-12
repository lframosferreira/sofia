use crate::parser::parser::Statement;
use crate::parser::token::Token;

#[derive(Debug, Clone)]
enum VarValue {
    Int64(i64),
    Float64(f64),
    Bool(bool),
    Str(String),
}

#[derive(Debug, Clone)]
struct Variable {
    name: String,
    value: VarValue,
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
            dbg!("oi");
            return element.clone();
        } else {
            panic!("not declared variable");
        }
    }
}

pub fn eval(code: Vec<Statement>) {}
