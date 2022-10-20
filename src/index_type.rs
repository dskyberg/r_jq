use crate::JQError;

/// Represents an identifier-index for objects and arrays. IndexType will have
/// either an identifier or an index.
#[derive(PartialEq, Debug, Clone)]
pub struct IndexType<'a> {
    identifier: Option<&'a str>,
    index: Option<Vec<isize>>,
    silent: bool,
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
    pub fn as_identifier(&self) -> Result<(&str, bool), JQError> {
        match self.identifier {
            Some(s) => Ok((s, self.silent)),
            _ => Err(JQError::BadIndexType),
        }
    }

    /// Return the index
    pub fn as_index(&self) -> Result<(Vec<isize>, bool), JQError> {
        match &self.index {
            Some(index) => Ok((index.to_vec(), self.silent)),
            _ => Err(JQError::BadIndexType),
        }
    }
}

impl<'a> From<(&'a str, bool)> for IndexType<'a> {
    fn from(id: (&'a str, bool)) -> Self {
        Self {
            identifier: Some(id.0),
            index: None,
            silent: id.1,
        }
    }
}

impl<'a> From<(isize, bool)> for IndexType<'a> {
    fn from(index: (isize, bool)) -> Self {
        Self {
            identifier: None,
            index: Some(vec![index.0]),
            silent: index.1,
        }
    }
}

impl<'a> From<(Vec<isize>, bool)> for IndexType<'a> {
    fn from(indexes: (Vec<isize>, bool)) -> Self {
        Self {
            identifier: None,
            index: Some(indexes.0),
            silent: indexes.1,
        }
    }
}
