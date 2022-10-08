use crate::{Function, IndexType, JQError, RangeType};

#[derive(PartialEq, Debug, Clone)]
pub enum Token<'a> {
    Identity,
    Ident(&'a str),
    Range(RangeType),
    Index(IndexType<'a>),
    // Filter(Vec<Token<'a>>),
}

impl<'a> Token<'a> {
    pub fn as_ident(&self) -> Result<&&'a str, JQError> {
        match self {
            Token::Ident(ident) => Ok(ident),
            _ => Err(JQError::TokenMismatch("Ident".to_string())),
        }
    }

    pub fn as_range(&self) -> Result<&RangeType, JQError> {
        match self {
            Token::Range(range) => Ok(range),
            _ => Err(JQError::TokenMismatch("Range".to_string())),
        }
    }

    pub fn as_index(&self) -> Result<&IndexType<'a>, JQError> {
        match self {
            Token::Index(index) => Ok(index),
            _ => Err(JQError::TokenMismatch("Key".to_string())),
        }
    }
    /*
        pub fn as_filter(&self) -> Result<Vec<Token>, JQError> {
            match self {
                Token::Filter(keys) => Ok(keys.to_vec()),
                _ => Err(JQError::TokenMismatch("Filter".to_string())),
            }
        }
    */
    pub fn is_identity(&self) -> bool {
        matches!(self, Token::Identity)
    }

    pub fn is_ident(&self) -> bool {
        matches!(self, Token::Ident(_))
    }
    pub fn is_index(&self) -> bool {
        matches!(self, Token::Index(_))
    }
    pub fn is_range(&self) -> bool {
        matches!(self, Token::Range(_))
    }
}

impl<'a> TryFrom<Function<'a>> for Token<'a> {
    type Error = JQError;
    fn try_from(command: Function<'a>) -> Result<Self, Self::Error> {
        match command {
            Function::Has { index, ident } => {
                if let Some(id) = ident {
                    return Ok(Token::Ident(id));
                }
                if let Some(idx) = index {
                    return Ok(Token::Index(IndexType::from_index(idx)));
                }
                Err(JQError::GeneralError("Token::try_from error".to_string()))
            }
            _ => Err(JQError::GeneralError("Token::try_from error".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_empty_has() {
        let has = Function::Has {
            index: None,
            ident: None,
        };
        let result = Token::try_from(has);
        assert!(result.is_err());
    }

    #[test]
    fn test_try_from_ident_has() {
        let has = Function::Has {
            index: None,
            ident: Some("elem1"),
        };
        let result = Token::try_from(has).expect("Failed");
        assert_eq!(result, Token::Ident("elem1"));
    }
    #[test]
    fn test_try_from_index_has() {
        let has = Function::Has {
            index: Some(0),
            ident: None,
        };
        let result = Token::try_from(has).expect("Failed");
        assert_eq!(result, Token::Index(IndexType::from_index(0)));
    }
}
