use crate::ast;

/// An expression which has been type-checked.
///
/// When a [`BoundExpression`] is being evaluated, it is guaranteed that all operations performed
/// are applied to operands of compatible types.
pub struct BoundExpression(pub ast::Expression);
