use crate::JQError;

/// Input for  [Has](crate::Function::Has)
///
/// Must have either an index or an ident.  Cannot have both.
#[derive(Clone, Debug, PartialEq)]
pub struct HasType<'a> {
    index: Option<isize>,
    ident: Option<&'a str>,
}

impl<'a> HasType<'a> {
    /// True if ident is Some
    pub fn is_ident(&self) -> bool {
        self.ident.is_some()
    }

    /// True if index is Some
    pub fn is_index(&self) -> bool {
        self.index.is_some()
    }

    /// Returns the index, or error if None.
    pub fn as_index(&self) -> Result<isize, JQError> {
        if self.index.is_none() {
            return Err(JQError::HasTypeError("No index".to_string()));
        }
        Ok(self.index.unwrap())
    }

    /// Returns the ident, or error if None
    pub fn as_ident(&self) -> Result<&'a str, JQError> {
        if self.ident.is_none() {
            return Err(JQError::HasTypeError("No ident".to_string()));
        }
        Ok(self.ident.unwrap())
    }
}

impl<'a> From<&'a str> for HasType<'a> {
    fn from(ident: &'a str) -> Self {
        Self {
            ident: Some(ident),
            index: None,
        }
    }
}

impl<'a> From<isize> for HasType<'a> {
    fn from(index: isize) -> Self {
        Self {
            ident: None,
            index: Some(index),
        }
    }
}
