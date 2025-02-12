use crate::parser::parser::{Declaration, Expr, Parameter, Statement};
use crate::parser::token::{
    ArithmeticOp, BinaryOp, CompareOp, Numeral, Reserved, Token, TokenType,
};

#[derive(Debug, Clone)]
enum Value {
    Int64(i64),
    UInt64(u64),
    Float64(f64),
    Bool(bool),
    Str(String),
    Func(Function),
    Nil,
}

impl Value {
    fn to_string(&self) -> String {
        match self {
            Value::Int64(i) => i.to_string(),
            Value::UInt64(u) => u.to_string(),
            Value::Float64(f) => f.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Str(s) => s.clone(),
            Value::Func(function) => function.name.to_string(),
            Value::Nil => "Nil".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct Variable {
    name: String,
    value: Value,
}

#[derive(Debug, Clone)]
struct Function {
    name: String,
    return_type: TokenType,
    parameters: Vec<Parameter>,
    body: Vec<Statement>,
}

// vec of variables which, in this case, are identifiers only
// vec of functions which, in this case, are identifiers only
struct Environment {
    variables: Vec<Variable>,
}

impl Environment {
    pub fn new() -> Self {
        return Environment { variables: vec![] };
    }

    pub fn insert_var(&mut self, name: &String, value: &Value) {
        self.variables.push(Variable {
            name: name.clone(),
            value: value.clone(),
        });
    }

    pub fn find_var(&self, name: &String) -> &Variable {
        if let Some(variable) = self.variables.iter().rfind(|&x| x.name == *name) {
            return variable;
        } else {
            panic!("Varibale {:?} not declared", name);
        }
    }

    fn find_var_mut(&mut self, name: &String) -> &mut Variable {
        if let Some(variable) = self.variables.iter_mut().rfind(|x| x.name == *name) {
            return variable;
        } else {
            panic!("Varibale {:?} not declared", name);
        }
    }

    pub fn change_var(&mut self, name: &String, value: &Value) {
        let mut variable = self.find_var_mut(name);
        variable.value = value.clone();
    }

    pub fn remove_last_n(&mut self, n: usize) {
        self.variables
            .truncate(self.variables.len().saturating_sub(n));
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
            Expr::Logical { op, lhs, rhs } => {
                let lhs_val = self.eval_expr(lhs).unwrap();
                let rhs_val = self.eval_expr(rhs).unwrap();
                use Value::*;
                match op._type {
                    TokenType::ReservedWord(Reserved::And) => {
                        match (lhs_val.clone(), rhs_val.clone()) {
                            (Bool(l), Bool(r)) => Some(Bool(l && r)),
                            _ => {
                                panic!("Can't use logical operator if both sides are not booleans")
                            }
                        }
                    }
                    TokenType::ReservedWord(Reserved::Or) => {
                        match (lhs_val.clone(), rhs_val.clone()) {
                            (Bool(l), Bool(r)) => Some(Bool(l || r)),
                            _ => {
                                panic!("Can't use logical operator if both sides are not booleans")
                            }
                        }
                    }
                    _ => None,
                }
            }
            Expr::UnaryExpr { op, child } => {
                if let Some(expr_val) = self.eval_expr(child) {
                    match op._type {
                        TokenType::ReservedWord(Reserved::Not) => {
                            if let Value::Bool(b) = expr_val {
                                return Some(Value::Bool(!b));
                            } else {
                                panic!("unary expression not must be used in boolean");
                            }
                        }
                        TokenType::BinaryOperator(BinaryOp::ArithmeticOperator(
                            ArithmeticOp::Minus,
                        )) => {
                            if let Value::Int64(i) = expr_val {
                                return Some(Value::Int64(-1 * i));
                            } else if let Value::Float64(f) = expr_val {
                                return Some(Value::Float64(-1.0 * f));
                            } else {
                                panic!("unary expression not must be used in boolean");
                            }
                        }
                        _ => panic!("The operator {:?} is not unary", op),
                    }
                }
                panic!("Expression child in unary does not evaluate to anything");
            }
            Expr::BinaryExpr { op, lhs, rhs } => {
                let lhs_val = self.eval_expr(lhs).unwrap();
                let rhs_val = self.eval_expr(rhs).unwrap();
                use Value::*;
                match op._type {
                    TokenType::BinaryOperator(BinaryOp::ArithmeticOperator(ArithmeticOp::Plus)) => {
                        match (lhs_val.clone(), rhs_val.clone()) {
                            (Int64(l), Int64(r)) => Some(Int64(l + r)),
                            (UInt64(l), UInt64(r)) => Some(UInt64(l + r)),
                            (Float64(l), Float64(r)) => Some(Float64(l + r)),
                            (Str(l), Str(r)) => Some(Str(format!("{}{}", l, r))),
                            _ => panic!("Can't sum value variant {:?} with {:?}", lhs_val, rhs_val),
                        }
                    }
                    TokenType::BinaryOperator(BinaryOp::ArithmeticOperator(
                        ArithmeticOp::Minus,
                    )) => match (lhs_val.clone(), rhs_val.clone()) {
                        (Int64(l), Int64(r)) => Some(Int64(l - r)),
                        (UInt64(l), UInt64(r)) => Some(UInt64(l - r)),
                        (Float64(l), Float64(r)) => Some(Float64(l - r)),
                        (Str(l), Str(r)) => Some(Str(format!("{}{}", l, r))),
                        _ => panic!(
                            "Can't use - (minus) in value variant {:?} with {:?}",
                            lhs_val, rhs_val
                        ),
                    },
                    TokenType::BinaryOperator(BinaryOp::ArithmeticOperator(
                        ArithmeticOp::Asterisk,
                    )) => match (lhs_val.clone(), rhs_val.clone()) {
                        (Int64(l), Int64(r)) => Some(Int64(l * r)),
                        (UInt64(l), UInt64(r)) => Some(UInt64(l * r)),
                        (Float64(l), Float64(r)) => Some(Float64(l * r)),
                        _ => panic!(
                            "Can't use * (multiply) in value variant {:?} with {:?}",
                            lhs_val, rhs_val
                        ),
                    },
                    TokenType::BinaryOperator(BinaryOp::ArithmeticOperator(
                        ArithmeticOp::Slash,
                    )) => match (lhs_val.clone(), rhs_val.clone()) {
                        (Int64(l), Int64(r)) => Some(Int64(l / r)),
                        (UInt64(l), UInt64(r)) => Some(UInt64(l / r)),
                        (Float64(l), Float64(r)) => Some(Float64(l / r)),
                        _ => panic!(
                            "Can't use / (division) in value variant {:?} with {:?}",
                            lhs_val, rhs_val
                        ),
                    },
                    TokenType::BinaryOperator(BinaryOp::ArithmeticOperator(
                        ArithmeticOp::Modulo,
                    )) => match (lhs_val.clone(), rhs_val.clone()) {
                        (Int64(l), Int64(r)) => Some(Int64(l % r)),
                        (UInt64(l), UInt64(r)) => Some(UInt64(l % r)),
                        _ => panic!(
                            "Can't use / (division) in value variant {:?} with {:?}",
                            lhs_val, rhs_val
                        ),
                    },
                    TokenType::BinaryOperator(BinaryOp::CompareOperator(CompareOp::Less)) => {
                        match (lhs_val.clone(), rhs_val.clone()) {
                            (Int64(l), Int64(r)) => Some(Bool(l < r)),
                            (UInt64(l), UInt64(r)) => Some(Bool(l < r)),
                            (Float64(l), Float64(r)) => Some(Bool(l < r)),
                            (Str(l), Str(r)) => Some(Bool(l < r)),
                            _ => panic!(
                                "Can't use < (less than) in value variant {:?} with {:?}",
                                lhs_val, rhs_val
                            ),
                        }
                    }
                    TokenType::BinaryOperator(BinaryOp::CompareOperator(CompareOp::Greater)) => {
                        match (lhs_val.clone(), rhs_val.clone()) {
                            (Int64(l), Int64(r)) => Some(Bool(l > r)),
                            (UInt64(l), UInt64(r)) => Some(Bool(l > r)),
                            (Float64(l), Float64(r)) => Some(Bool(l > r)),
                            (Str(l), Str(r)) => Some(Bool(l > r)),
                            _ => panic!(
                                "Can't use > (greater than) in value variant {:?} with {:?}",
                                lhs_val, rhs_val
                            ),
                        }
                    }
                    TokenType::BinaryOperator(BinaryOp::CompareOperator(CompareOp::LessEqual)) => {
                        match (lhs_val.clone(), rhs_val.clone()) {
                            (Int64(l), Int64(r)) => Some(Bool(l <= r)),
                            (UInt64(l), UInt64(r)) => Some(Bool(l <= r)),
                            (Float64(l), Float64(r)) => Some(Bool(l <= r)),
                            (Str(l), Str(r)) => Some(Bool(l <= r)),
                            _ => panic!(
                                "Can't use <= (less than or equal) in value variant {:?} with {:?}",
                                lhs_val, rhs_val
                            ),
                        }
                    }
                    TokenType::BinaryOperator(BinaryOp::CompareOperator(
                        CompareOp::GreaterEqual,
                    )) => match (lhs_val.clone(), rhs_val.clone()) {
                        (Int64(l), Int64(r)) => Some(Bool(l >= r)),
                        (UInt64(l), UInt64(r)) => Some(Bool(l >= r)),
                        (Float64(l), Float64(r)) => Some(Bool(l >= r)),
                        (Str(l), Str(r)) => Some(Bool(l >= r)),
                        _ => panic!(
                            "Can't use >= (grater than or equal) in value variant {:?} with {:?}",
                            lhs_val, rhs_val
                        ),
                    },
                    TokenType::BinaryOperator(BinaryOp::CompareOperator(CompareOp::EqualEqual)) => {
                        match (lhs_val.clone(), rhs_val.clone()) {
                            (Int64(l), Int64(r)) => Some(Bool(l == r)),
                            (UInt64(l), UInt64(r)) => Some(Bool(l == r)),
                            (Float64(l), Float64(r)) => Some(Bool(l == r)),
                            (Str(l), Str(r)) => Some(Bool(l == r)),
                            (Bool(l), Bool(r)) => Some(Bool(l == r)),
                            _ => panic!(
                                "Can't use == (equal equal) in value variant {:?} with {:?}",
                                lhs_val, rhs_val
                            ),
                        }
                    }
                    TokenType::BinaryOperator(BinaryOp::CompareOperator(CompareOp::BangEqual)) => {
                        match (lhs_val.clone(), rhs_val.clone()) {
                            (Int64(l), Int64(r)) => Some(Bool(l != r)),
                            (UInt64(l), UInt64(r)) => Some(Bool(l != r)),
                            (Float64(l), Float64(r)) => Some(Bool(l != r)),
                            (Str(l), Str(r)) => Some(Bool(l != r)),
                            (Bool(l), Bool(r)) => Some(Bool(l == r)),
                            _ => panic!(
                                "Can't use != (bang equal) in value variant {:?} with {:?}",
                                lhs_val, rhs_val
                            ),
                        }
                    }
                    _ => None,
                }
            }
            Expr::Variable { value } => {
                let identifier = value.value.clone().unwrap();
                let variable = self.env.find_var(&identifier);
                Some(variable.value.clone())
            }
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                enum VarType {
                    Bool,
                    Int,
                    Float,
                    Str,
                    Func,
                    Nil,
                }
                if let Some(callee_value) = self.eval_expr(callee) {
                    if let Value::Func(func) = callee_value {
                        if arguments.len() != func.parameters.len() {
                            panic!("Arguments list and parameters list doesn't have the same size");
                        }
                        let arguments_evaluated: Vec<_> = arguments
                            .iter()
                            .map(|arg| self.eval_expr(arg).unwrap_or(Value::Nil))
                            .collect();
                        let parameters_types: Vec<_> = func
                            .parameters
                            .iter()
                            .map(|p| match p.type_ {
                                TokenType::ReservedWord(Reserved::Bool) => VarType::Bool,
                                TokenType::ReservedWord(Reserved::Int) => VarType::Int,
                                TokenType::ReservedWord(Reserved::Float) => VarType::Float,
                                TokenType::ReservedWord(Reserved::String) => VarType::Str,
                                TokenType::Nil => VarType::Nil,
                                _ => unreachable!(),
                            })
                            .collect();
                        if arguments_evaluated
                            .iter()
                            .zip(parameters_types)
                            .map(|val| match val {
                                (Value::Bool(_), VarType::Bool) => true,
                                (Value::Int64(_), VarType::Int) => true,
                                (Value::Float64(_), VarType::Float) => true,
                                (Value::Str(_), VarType::Str) => true,
                                (Value::Nil, VarType::Nil) => true,
                                _ => false,
                            })
                            .any(|v| v == false)
                        {
                            panic!("Values passed to function doesn't fit the parameter types of the functions");
                        }
                        arguments_evaluated.iter().enumerate().for_each(|(i, arg)| {
                            self.env.insert_var(&func.parameters[i].identifier, arg);
                        });
                        let mut ret: Option<Value> = None;
                        let mut decl_count: usize = 0;
                        for stmt in func.body {
                            match stmt {
                                Statement::ReturnStatement { expr } => {
                                    if let Some(expr_unwrapped) = expr {
                                        ret = self.eval_expr(&expr_unwrapped);
                                    } else {
                                        ret = Some(Value::Nil);
                                    }
                                }
                                _ => {
                                    if let Statement::Declaration(_) = stmt {
                                        decl_count += 1;
                                    }
                                    self.eval_stmt(&stmt);
                                }
                            }
                        }
                        // Now I need to drop the variables declared in this scope, which are the
                        // ones in the functions scope and the parameters
                        self.env.remove_last_n(decl_count + arguments.len());
                        ret
                    } else {
                        panic!("Callee expression doesn't evaluate to a function");
                    }
                } else {
                    panic!("Callee expression doesn't evaluate to anything");
                }
            }
            Expr::Assign { name, value } => {
                if let Some(expr_value) = self.eval_expr(value) {
                    self.env
                        .change_var(&name.value.clone().unwrap(), &expr_value);
                    Some(expr_value)
                } else {
                    panic!("Expression in assignment doesn't evaluate to anything, should be NIL in future");
                }
            }
            _ => None,
        }
    }

    pub fn eval_decl(&mut self, declaration: &Declaration) -> Option<Value> {
        match declaration {
            Declaration::VariableDeclaration {
                name,
                type_,
                initializer_expr,
            } => {
                if let Some(expr) = initializer_expr {
                    if let Some(expr_value) = self.eval_expr(expr) {
                        self.env
                            .insert_var(&name.value.clone().unwrap(), &expr_value);
                        Some(expr_value)
                    } else {
                        panic!("Expression in assignment doesn't evaluate to anything, should be NIL in future");
                    }
                } else {
                    self.env
                        .insert_var(&name.value.clone().unwrap(), &Value::Nil);
                    None
                }
            }
            Declaration::FunctionDeclaration {
                name,
                return_type,
                parameters,
                body,
            } => {
                self.env.insert_var(
                    &name.value.clone().unwrap(),
                    &Value::Func(Function {
                        name: name.value.clone().unwrap(),
                        return_type: return_type._type.clone(),
                        parameters: parameters.to_vec(),
                        body: body.to_vec(),
                    }),
                );
                None
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
            Statement::WhileStatement { expr, stmt } => {
                use Value::*;
                loop {
                    if let Some(expr_val) = self.eval_expr(expr) {
                        if let Some(expr_b) = match expr_val {
                            Bool(b) => Some(b),
                            _ => None,
                        } {
                            if expr_b {
                                self.eval_stmt(stmt);
                            } else {
                                break;
                            }
                        } else {
                            panic!(
                                "Expression inside while statement should evaluate to a boolean"
                            );
                        }
                    } else {
                        panic!("Expression inside while statement should evaluate to something");
                    }
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
                        Value::Nil => println!("Nil"),
                        Value::Func(function) => println!("{:?}", function.name.to_string()),
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
