use crate::parser::token::*;

pub enum Node {
    UnaryExpr{op: CompareOperator, lhs: Box<Node>}
}
