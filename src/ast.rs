use crate::lexer::Token;

pub struct IntegerLiteral(pub i32);

pub enum BinaryOperation {
    Plus,
    Minus,
    Times,
    Divided,
}

impl BinaryOperation {
    pub fn from_token(token: Token) -> Option<Self> {
        match token {
            Token::Plus => Some(Self::Plus),
            Token::Minus => Some(Self::Minus),
            Token::Star => Some(Self::Times),
            Token::Slash => Some(Self::Divided),
            _ => None,
        }
    }
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
