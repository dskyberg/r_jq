use crate::{AtomType, Operator};

/// Represents an expression such as `l == r`
#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionType<'a> {
    ///
    pub left: AtomType<'a>,
    ///
    pub operator: Operator,
    ///
    pub right: AtomType<'a>,
}
