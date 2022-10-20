use crate::{Function, HasType, IndexType, JQError, RangeType};

/// Tokens are the components in a Filter query
#[derive(PartialEq, Debug, Clone)]
pub enum Token<'a> {
    /// I`.`, represents the entire input
    Identity,
    /// `<str>`, represents an object key.  Bool repeents whether to return errors
    Ident(&'a str, bool),
    ///  `[isize:isize]`, represents an array slice
    Range(RangeType),
    /// `[isize]`, represents an array index
    Index(IndexType<'a>),
}

impl<'a> Token<'a> {
    /// Returns the ident component of a Token, or error
    pub fn as_ident(&self) -> Result<(&'a str, bool), JQError> {
        match self {
            Token::Ident(ident, silent) => Ok((ident, *silent)),
            _ => Err(JQError::TokenMismatch("Ident".to_string())),
        }
    }

    /// Returns the range or error
    pub fn as_range(&self) -> Result<&RangeType, JQError> {
        match self {
            Token::Range(range) => Ok(range),
            _ => Err(JQError::TokenMismatch("Range".to_string())),
        }
    }

    /// Returns the index or error
    pub fn as_index(&self) -> Result<&IndexType<'a>, JQError> {
        match self {
            Token::Index(index) => Ok(index),
            _ => Err(JQError::TokenMismatch("Key".to_string())),
        }
    }

    /// True the Token is an Identity
    pub fn is_identity(&self) -> bool {
        matches!(self, Token::Identity)
    }

    /// True the Token is an Ident
    pub fn is_ident(&self) -> bool {
        matches!(self, Token::Ident(..))
    }

    /// True the Token is an Index
    pub fn is_index(&self) -> bool {
        matches!(self, Token::Index(_))
    }

    /// True the Token is a Range
    pub fn is_range(&self) -> bool {
        matches!(self, Token::Range(_))
    }
}

impl<'a> From<&HasType<'a>> for Token<'a> {
    /// HasType is guaranteed to have either an ident or an index.
    fn from(has: &HasType<'a>) -> Self {
        if has.is_ident() {
            return Token::Ident(has.as_ident().unwrap(), false);
        }
        Token::Index(IndexType::from((has.as_index().unwrap(), false)))
    }
}

impl<'a> TryFrom<Function<'a>> for Token<'a> {
    type Error = JQError;
    fn try_from(command: Function<'a>) -> Result<Self, Self::Error> {
        match command {
            Function::Has(has) => Ok(Self::from(&has)),
            _ => Err(JQError::GeneralError("Token::try_from error".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HasType;

    #[test]
    fn test_try_from_ident_has() {
        let has = HasType::from("elem1");
        let result = Token::try_from(&has).expect("Failed");
        assert_eq!(result, Token::Ident("elem1", false));
    }
    #[test]
    fn test_try_from_index_has() {
        let has = HasType::from(0);

        let result = Token::try_from(&has).expect("failed");
        assert_eq!(result, Token::Index(IndexType::from((0, false))));
    }
}
