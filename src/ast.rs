use crate::lexer::Token;

#[derive(Clone, Copy)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Times,
    Divided,
}

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Times => write!(f, "*"),
            Self::Divided => write!(f, "/"),
        }
    }
}

impl BinaryOperator {
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

pub enum Expression {
    IntegerLiteral(i32),
    Binary {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

pub fn pretty_print(expr: &Expression) {
    fn aux(expr: &Expression, indent: &str, is_last: bool) {
        let branch = if is_last { "└── " } else { "├── " };
        match expr {
            Expression::IntegerLiteral(value) => println!("{indent}{branch}IntegerLiteral {value}"),
            Expression::Binary {
                operator,
                left,
                right,
            } => {
                let new_indent = if is_last {
                    format!("{indent}    ")
                } else {
                    format!("{indent}│   ")
                };

                println!("{indent}{branch}BinaryExpression {operator}");
                aux(left.as_ref(), &new_indent, false);
                aux(right.as_ref(), &new_indent, true);
            }
        }
    }

    println!(".");
    aux(expr, "", true);
}
