use crate::ast::{BinaryOperator, Expression};

pub fn evaluate(expr: Expression) -> i32 {
    match expr {
        Expression::IntegerLiteral(value) => value,
        Expression::Binary {
            operator,
            left,
            right,
        } => evaluate_binary(operator, *left, *right),
    }
}

pub fn evaluate_binary(operator: BinaryOperator, left: Expression, right: Expression) -> i32 {
    let lhs = evaluate(left);
    let rhs = evaluate(right);

    match operator {
        BinaryOperator::Plus => lhs + rhs,
        BinaryOperator::Minus => lhs - rhs,
        BinaryOperator::Times => lhs * rhs,
        BinaryOperator::Divided => lhs / rhs,
    }
}
