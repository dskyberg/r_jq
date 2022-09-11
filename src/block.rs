use crate::Token;

#[derive(Debug, PartialEq)]
pub struct Block<'a> {
    pub filters: Option<Vec<Token<'a>>>,
}
