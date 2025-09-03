use std::iter::Peekable;

use crate::{
    ast::{BinaryExpression, BinaryOperation, Expression, IntegerLiteral},
    lexer::Token,
};

pub struct Parser<I>
where
    I: Iterator,
{
    tokens: Peekable<I>,
}

impl<I> Parser<I>
where
    I: Iterator,
{
    pub fn new(x: impl IntoIterator<IntoIter = I>) -> Self {
        Self {
            tokens: x.into_iter().peekable(),
        }
    }
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    pub fn parse(&mut self) -> Expression {
        let expr = self.parse_expression();
        match self.tokens.next() {
            Some(Token::EndOfFile) => expr,
            _ => panic!("expected end of file"),
        }
    }

    pub fn parse_expression(&mut self) -> Expression {
        self.parse_term()
    }

    pub fn parse_term(&mut self) -> Expression {
        let mut left = self.parse_factor();

        while let Some(token) = self.tokens.peek() {
            let (Token::Plus | Token::Minus) = token else {
                break;
            };

            // This unwrap call is safe because next() will always return the same variant as
            // peek(). We would not be in this iteration if peek() did not return Some, so we are
            // good to go.
            // Additionally, the unwrap call on from_token() is guaranteed not to panic since token
            // must be either Plus or Minus, and those can, without a doubt, be converted to a
            // BinaryOperation.
            let operator = self.tokens.next().unwrap();
            let right = self.parse_factor();
            left = Expression::Binary(BinaryExpression {
                operator: BinaryOperation::from_token(operator).unwrap(),
                left: Box::new(left),
                right: Box::new(right),
            });
        }

        left
    }

    pub fn parse_factor(&mut self) -> Expression {
        let mut left = self.parse_primary();

        while let Some(token) = self.tokens.peek() {
            let (Token::Star | Token::Slash) = token else {
                break;
            };

            // These unwrap calls are safe for the same reasons as above.
            let operator = self.tokens.next().unwrap();
            let right = self.parse_primary();
            left = Expression::Binary(BinaryExpression {
                operator: BinaryOperation::from_token(operator).unwrap(),
                left: Box::new(left),
                right: Box::new(right),
            });
        }

        left
    }

    pub fn parse_primary(&mut self) -> Expression {
        let token = self.tokens.next().unwrap();
        match token {
            Token::ParenLeft => {
                let expr = self.parse_expression();
                let Some(Token::ParenRight) = self.tokens.next() else {
                    panic!("expected ')' after expression");
                };
                expr
            }
            Token::Number(value) => Expression::Integer(IntegerLiteral(value)),
            _ => panic!("unexpected token"),
        }
    }
}
