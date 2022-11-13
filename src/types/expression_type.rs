use crate::{Operator, Token};

///
#[derive(Clone, PartialEq, Debug)]
pub enum ExpressionType<'a> {
    ///
    Number(f64),
    ///
    String(&'a str),
    ///
    Ident(Token<'a>),
    ///
    Op(Operator, Box<ExpressionType<'a>>, Box<ExpressionType<'a>>),
}
