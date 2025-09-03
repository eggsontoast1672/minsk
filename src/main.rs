use minsk::{
    ast::{BinaryExpression, BinaryOperation, Expression, IntegerLiteral},
    eval,
};

fn main() {
    let expr = BinaryExpression {
        operator: BinaryOperation::Plus,
        left: Box::new(Expression::Integer(IntegerLiteral(1))),
        right: Box::new(Expression::Binary(BinaryExpression {
            operator: BinaryOperation::Times,
            left: Box::new(Expression::Integer(IntegerLiteral(2))),
            right: Box::new(Expression::Integer(IntegerLiteral(3))),
        })),
    };

    let result = eval::evaluate_binary(expr);

    println!("result = {result}");
}
