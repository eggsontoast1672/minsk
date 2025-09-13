use crate::lexer::Token;

#[derive(Clone)]
pub enum LiteralKind {
    Integer,
    Boolean,
}

#[derive(Clone)]
pub struct LiteralExpression {
    pub kind: LiteralKind,
    pub token: Token,
}

#[derive(Clone)]
pub struct UnaryExpression {
    pub operator: Token,
    pub right: Box<Expression>,
}

#[derive(Clone)]
pub struct BinaryExpression {
    pub operator: Token,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Clone)]
pub enum Expression {
    Literal(LiteralExpression),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
}

pub fn pretty_print(expr: &Expression) {
    fn aux(expr: &Expression, indent: &str, is_last: bool) {
        let branch = if is_last { "└── " } else { "├── " };
        match expr {
            Expression::Literal(Literal::Integer(n)) => {
                println!("{indent}{branch}IntegerLiteral {n}");
            }
            Expression::Literal(Literal::Boolean(b)) => {
                println!("{indent}{branch}BooleanLiteral {b}");
            }
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
