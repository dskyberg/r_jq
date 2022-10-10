/// Actions are a collection of filters and functions that
/// can be executed within a block.  A collection of Actions
/// can be sequentially processed to both filter and transform
/// input.
use crate::{Function, JQError, Token};

/// A Filter is just a collection of Tokens
pub type Filter<'a> = Vec<Token<'a>>;

/// An action is the fundamental component of a [Block](crate::Block)
#[derive(Clone, Debug, PartialEq)]
pub enum Action<'a> {
    /// [Function]
    Function(Function<'a>),
    /// [Filter]
    Filter(Filter<'a>),
}

impl<'a> Action<'a> {
    /// Return the inner [Function], or error
    pub fn as_function(&self) -> Result<&Function, JQError> {
        match self {
            Action::Function(function) => Ok(function),
            _ => Err(JQError::ActionMismatch("Function".to_string())),
        }
    }

    /// Return the inner [Filter], or error
    pub fn as_filter(&self) -> Result<&Filter, JQError> {
        match self {
            Action::Filter(filter) => Ok(filter),
            _ => Err(JQError::ActionMismatch("Filter".to_string())),
        }
    }

    /// True if the [Action] is a [Function]
    pub fn is_function(&self) -> bool {
        matches!(self, Action::Function(_))
    }

    /// True if the [Action] is a [Filter]
    pub fn is_filter(&self) -> bool {
        matches!(self, Action::Filter(_))
    }
}

impl<'a> From<Function<'a>> for Action<'a> {
    fn from(function: Function<'a>) -> Self {
        Self::Function(function)
    }
}

impl<'a> From<Vec<Token<'a>>> for Action<'a> {
    fn from(tokens: Vec<Token<'a>>) -> Self {
        Self::Filter(tokens)
    }
}

impl<'a> From<Token<'a>> for Action<'a> {
    fn from(token: Token<'a>) -> Self {
        Self::Filter(vec![token])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_function() {
        let length = Function::Length;
        let action = Action::from(length);
        assert_eq!(action, Action::Function(Function::Length));
    }

    #[test]
    fn test_from_token() {
        let filter = vec![Token::Identity, Token::Ident(".something", false)];
        let action = Action::from(filter);
        assert_eq!(
            action,
            Action::Filter(vec![Token::Identity, Token::Ident(".something", false)])
        )
    }
}
