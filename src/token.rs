use crate::{IndexType, JQError, RangeType};

#[derive(PartialEq, Debug, Clone)]
pub enum Token<'a> {
    Identity,
    Ident(&'a str),
    Range(RangeType),
    Index(IndexType<'a>),
    Filter(Vec<Token<'a>>),
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

    pub fn as_filter(&self) -> Result<Vec<Token>, JQError> {
        match self {
            Token::Filter(keys) => Ok(keys.to_vec()),
            _ => Err(JQError::TokenMismatch("Filter".to_string())),
        }
    }

    pub fn is_identity(&self) -> bool {
        match self {
            Token::Identity => true,
            Token::Filter(filters) => filters.len() == 1 && filters[0].is_identity(),
            _ => false,
        }
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
