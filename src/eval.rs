use crate::{ast, lexer::TokenKind};

pub enum Value {
    Integer(i32),
    Boolean(bool),
}

pub fn evaluate(expr: ast::Expression) -> Value {
    match expr {
        ast::Expression::Literal(literal) => evaluate_literal(literal),
        ast::Expression::Unary(unary) => evaluate_unary(unary),
        ast::Expression::Binary(binary) => evaluate_binary(binary),
    }
}

pub fn evaluate_literal(expr: ast::LiteralExpression) -> Value {
    match expr.kind {
        ast::LiteralKind::Integer => Value::Integer(),
        ast::LiteralKind::Boolean => todo!(),
    }
}

fn evaluate_unary(expr: ast::UnaryExpression) -> ast::LiteralExpression {
    match expr.operator.kind {
        TokenKind::Bang => evalute_unary_not(*expr.right),
        TokenKind::Minus => evaluate_unary_minus(*expr.right),
        TokenKind::Plus => evaluate_unary_plus(*expr.right),
        _ => panic!("invalid operator for unary expression"),
    }
}

fn evaluate_unary_not(expr: ast::Expression) -> ast::LiteralExpression {
    let right = evaluate(expr);
    match right.kind {
        ast::LiteralKind::Boolean => ast::LiteralExpression {},
    }
}

fn evaluate_unary_minus(expr: ast::Expression) -> ast::LiteralExpression {
    todo!()
}

fn evaluate_unary_plus(expr: ast::Expression) -> ast::LiteralExpression {
    todo!()
}

fn evaluate_binary(expr: ast::BinaryExpression) -> ast::LiteralExpression {
    todo!()
}
