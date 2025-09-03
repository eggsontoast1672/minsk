pub struct IntegerLiteral(pub i32);

pub enum BinaryOperation {
    Plus,
    Minus,
    Times,
    Divided,
}

pub struct BinaryExpression {
    pub operator: BinaryOperation,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

pub enum Expression {
    Integer(IntegerLiteral),
    Binary(BinaryExpression),
}
