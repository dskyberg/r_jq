use crate::{JQError, KeyType, Range};

#[derive(PartialEq, Debug, Clone)]
pub enum Token<'a> {
    Ident(&'a str),
    Iterator(Option<Range>),
    Key(KeyType<'a>),
    Filter(Vec<Token<'a>>),
}

impl<'a> Token<'a> {
    pub fn as_ident(&self) -> Result<&&'a str, JQError> {
        match self {
            Token::Ident(ident) => Ok(ident),
            _ => Err(JQError::TokenMismatch("Ident".to_string())),
        }
    }

    pub fn as_iterator(&self) -> Result<&Option<Range>, JQError> {
        match self {
            Token::Iterator(range) => Ok(range),
            _ => Err(JQError::TokenMismatch("Iterator".to_string())),
        }
    }

    pub fn as_key(&self) -> Result<&KeyType<'a>, JQError> {
        match self {
            Token::Key(key_type) => Ok(key_type),
            _ => Err(JQError::TokenMismatch("Key".to_string())),
        }
    }

    pub fn as_filter(&self) -> Result<Vec<Token>, JQError> {
        match self {
            Token::Filter(keys) => Ok(keys.to_vec()),
            _ => Err(JQError::TokenMismatch("Filter".to_string())),
        }
    }

    pub fn is_identity(&self) -> Result<bool, JQError> {
        let keys = self.as_filter()?;
        if keys.len() != 1 {
            return Ok(false);
        }
        let key = keys[0].as_key()?;
        Ok(key.is_identity())
    }
}
