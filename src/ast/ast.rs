use crate::parser::token::BinaryOp;

pub enum Node {
    BinaryExpr {
        op: BinaryOp,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}
