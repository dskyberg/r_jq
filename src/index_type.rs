use crate::JQError;

/// Represents an identifier-index for objects and arrays. IndexType will have
/// either an identifier or an index.
#[derive(PartialEq, Debug, Clone)]
pub struct IndexType<'a> {
    identifier: Option<&'a str>,
    index: Option<Vec<isize>>,
}

impl<'a> IndexType<'a> {
    /// True if identifier is Some
    pub fn is_identitfier(&self) -> bool {
        self.identifier.is_some()
    }

    /// True if index is Some
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

    /// Return the identifier
    pub fn as_identifier(&self) -> Result<&str, JQError> {
        match self.identifier {
            Some(s) => Ok(s),
            _ => Err(JQError::BadIndexType),
        }
    }

    /// Return the index
    pub fn as_index(&self) -> Result<Vec<isize>, JQError> {
        match &self.index {
            Some(index) => Ok(index.to_vec()),
            _ => Err(JQError::BadIndexType),
        }
    }
}

impl<'a> From<&'a str> for IndexType<'a> {
    fn from(id: &'a str) -> Self {
        Self {
            identifier: Some(id),
            index: None,
        }
    }
}

impl<'a> From<isize> for IndexType<'a> {
    fn from(index: isize) -> Self {
        Self {
            identifier: None,
            index: Some(vec![index]),
        }
    }
}

impl<'a> From<Vec<isize>> for IndexType<'a> {
    fn from(indexes: Vec<isize>) -> Self {
        Self {
            identifier: None,
            index: Some(indexes),
        }
    }
}
