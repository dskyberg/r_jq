use crate::Action;

/// A block represents a collection of [actions](Action), separated by a pipe
/// `<action 1> | <action 2>`
#[derive(Clone, Debug, PartialEq)]
pub struct Block<'a> {
    /// Collection of [Action] to be processed
    pub actions: Option<Vec<Action<'a>>>,
    /// Collect the action results in an array
    pub collect: bool,
}

impl<'a> From<Action<'a>> for Block<'a> {
    fn from(action: Action<'a>) -> Self {
        Self {
            actions: Some(vec![action]),
            collect: false,
        }
    }
}

impl<'a> From<Vec<Action<'a>>> for Block<'a> {
    fn from(actions: Vec<Action<'a>>) -> Self {
        Self {
            actions: Some(actions),
            collect: false,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Token;

    #[test]
    fn block_from_action() {
        let action = Action::from(Token::Identity);
        let block = Block::from(action);
        assert_eq!(
            block,
            Block {
                actions: Some(vec![Action::Filter(vec![Token::Identity])]),
                collect: false,
            }
        );
    }

    #[test]
    fn block_from_actions() {
        let action1 = Action::from(Token::Identity);
        let action2 = Action::from(Token::Ident("elem1", false));
        let block = Block::from(vec![action1, action2]);
        assert_eq!(
            block,
            Block {
                actions: Some(vec![
                    Action::Filter(vec![Token::Identity]),
                    Action::Filter(vec![Token::Ident("elem1", false)])
                ]),
                collect: false
            }
        );
    }
}
