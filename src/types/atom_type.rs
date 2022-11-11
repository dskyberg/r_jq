use crate::{ExpressionType, Token};

///
#[derive(Clone, Debug, PartialEq)]
pub enum AtomType<'a> {
    ///
    String(&'a str),
    ///
    Number(f64),
    ///
    Token(&'a Token<'a>),
    ///
    Expression(&'a ExpressionType<'a>),
}

impl<'a> AtomType<'a> {
    ///
    pub fn is_string(&self) -> bool {
        matches!(self, AtomType::String(_))
    }
    ///
    pub fn is_number(&self) -> bool {
        matches!(self, AtomType::Number(_))
    }
    ///
    pub fn is_token(&self) -> bool {
        matches!(self, AtomType::Token(_))
    }
    ///
    pub fn is_expression(&self) -> bool {
        matches!(self, AtomType::Expression(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is() {
        let a = AtomType::String("abc");
        assert!(a.is_string());
    }
}
