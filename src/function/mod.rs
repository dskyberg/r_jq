pub mod has;

#[derive(Clone, Debug, PartialEq)]
pub enum Function<'a> {
    Length,
    Has {
        index: Option<isize>,
        ident: Option<&'a str>,
    },
}

impl<'a> Function<'a> {
    pub fn is_has(&self) -> bool {
        matches!(self, Function::Has { index: _, ident: _ })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_has() {
        let has = Function::Has {
            index: None,
            ident: None,
        };
        assert!(has.is_has());
    }
}
