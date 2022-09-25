use crate::JQError;

#[derive(PartialEq, Debug, Clone)]
pub struct IndexType<'a> {
    pub identifier: Option<&'a str>,
    pub index: Option<isize>,
}

impl<'a> IndexType<'a> {
    pub fn new() -> Self {
        Self {
            identifier: None,
            index: None,
        }
    }

    pub fn from_identifier(id: &'a str) -> Self {
        Self {
            identifier: Some(id),
            index: None,
        }
    }

    pub fn from_index(index: isize) -> Self {
        Self {
            identifier: None,
            index: Some(index),
        }
    }
    pub fn is_identitfier(&self) -> bool {
        self.identifier.is_some()
    }

    pub fn is_index(&self) -> bool {
        self.index.is_some()
    }

    /// When Identity is not allowed, a IndexType must have either
    /// an identifier or a range, and not both.
    pub fn is_valid(&self) -> bool {
        if self.identifier.is_some() && self.index.is_none() {
            return true;
        }

        if self.index.is_some() && self.identifier.is_none() {
            return true;
        }
        false
    }

    pub fn as_identifier(&self) -> Result<&str, JQError> {
        match self.identifier {
            Some(s) => Ok(s),
            _ => Err(JQError::BadIndexType),
        }
    }

    pub fn as_index(&self) -> Result<isize, JQError> {
        match self.index {
            Some(index) => Ok(index),
            _ => Err(JQError::BadIndexType),
        }
    }
}

impl<'a> Default for IndexType<'a> {
    fn default() -> Self {
        Self::new()
    }
}
