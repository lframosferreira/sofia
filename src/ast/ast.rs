use crate::parser::token::BinaryOp;
use crate::parser::token::Numeral::{Float64, Int64};

pub enum Node {
    BinaryExpr {
        op: BinaryOp,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Int(Int64),
    Float(Float64),
    String(String),
}
