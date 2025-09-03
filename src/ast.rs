use crate::lexer::TokenKind;

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
    pub fn from_token_kind(kind: TokenKind) -> Option<Self> {
        match kind {
            TokenKind::Plus => Some(Self::Plus),
            TokenKind::Minus => Some(Self::Minus),
            TokenKind::Star => Some(Self::Times),
            TokenKind::Slash => Some(Self::Divided),
            _ => None,
        }
    }
}

#[derive(Clone)]
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
