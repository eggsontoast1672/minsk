use std::iter::Peekable;

use crate::{
    ast::{BinaryOperator, Expression},
    lexer::{Token, TokenKind},
};

#[derive(Debug)]
pub enum ParseError {
    Syntax,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Syntax => write!(f, "syntax error"),
        }
    }
}

impl std::error::Error for ParseError {}

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
    pub fn peek_kind(&mut self) -> Option<&TokenKind> {
        self.tokens.peek().map(|t| &t.kind)
    }

    pub fn parse(&mut self) -> Result<Expression, ParseError> {
        let expr = self.parse_expression()?;
        match self.tokens.next() {
            Some(Token {
                kind: TokenKind::EndOfFile,
                length: _,
            }) => Ok(expr),
            _ => Err(ParseError::Syntax),
        }
    }

    pub fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_term()
    }

    pub fn parse_term(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_factor()?;

        while let Some(TokenKind::Plus | TokenKind::Minus) = self.peek_kind() {
            // This unwrap call is safe because next() will always return the same variant as
            // peek(). We would not be in this iteration if peek() did not return Some, so we are
            // good to go.
            // Additionally, the unwrap call on from_token() is guaranteed not to panic since token
            // must be either Plus or Minus, and those can, without a doubt, be converted to a
            // BinaryOperation.
            let operator = self.tokens.next().unwrap();
            let right = self.parse_factor()?;
            left = Expression::Binary {
                operator: BinaryOperator::from_token_kind(operator.kind).unwrap(),
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    pub fn parse_factor(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_primary()?;

        while let Some(TokenKind::Star | TokenKind::Slash) = self.peek_kind() {
            // These unwrap calls are safe for the same reasons as above.
            let operator = self.tokens.next().unwrap();
            let right = self.parse_primary()?;
            left = Expression::Binary {
                operator: BinaryOperator::from_token_kind(operator.kind).unwrap(),
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    pub fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        let token = self.tokens.next().unwrap();
        match token.kind {
            TokenKind::ParenLeft => {
                let expr = self.parse_expression()?;
                match self.tokens.next() {
                    Some(Token {
                        kind: TokenKind::ParenRight,
                        length: _,
                    }) => Ok(expr),
                    _ => Err(ParseError::Syntax),
                }
            }
            TokenKind::Number(value) => Ok(Expression::IntegerLiteral(value)),
            _ => Err(ParseError::Syntax),
        }
    }
}
