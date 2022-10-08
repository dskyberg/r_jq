use crate::Action;

#[derive(Clone, Debug, PartialEq)]
pub struct Block<'a> {
    pub actions: Option<Vec<Action<'a>>>,
}
