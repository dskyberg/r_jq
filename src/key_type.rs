use crate::Range;

#[derive(PartialEq, Debug, Clone)]
pub struct KeyType<'a> {
    pub identifier: Option<&'a str>,
    pub range: Option<Range>,
}

impl<'a> KeyType<'a> {
    pub fn new() -> Self {
        Self {
            identifier: None,
            range: None,
        }
    }
    pub fn from_identifier(id: &'a str) -> Self {
        Self {
            identifier: Some(id),
            range: None,
        }
    }

    pub fn from_range(range: Range) -> Self {
        Self {
            identifier: None,
            range: Some(range),
        }
    }

    /// Neither identifier nor range are provided.
    /// This is only valid as the first and only key in a filter.
    pub fn is_identity(&self) -> bool {
        self.identifier.is_none()
    }

    /// When Identity is not allowed, a KeyType must have either
    /// an identifier or a range, and not both.
    pub fn is_valid(&self) -> bool {
        if self.identifier.is_some() && self.range.is_none() {
            return true;
        }
        if self.range.is_some() && self.identifier.is_none() {
            return true;
        }
        false
    }
}
impl<'a> Default for KeyType<'a> {
    fn default() -> Self {
        Self::new()
    }
}
