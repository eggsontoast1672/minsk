use crate::ast::{BinaryExpression, BinaryOperation, Expression};

pub fn evaluate(expr: Expression) -> i32 {
    match expr {
        Expression::Integer(integer) => integer.0,
        Expression::Binary(binary) => evaluate_binary(binary),
    }
}

pub fn evaluate_binary(expr: BinaryExpression) -> i32 {
    let lhs = evaluate(*expr.left);
    let rhs = evaluate(*expr.right);

    match expr.operator {
        BinaryOperation::Plus => lhs + rhs,
        BinaryOperation::Minus => lhs - rhs,
        BinaryOperation::Times => lhs * rhs,
        BinaryOperation::Divided => lhs / rhs,
    }
}
