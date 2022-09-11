#[derive(PartialEq, Debug)]
pub enum Command<'a> {
    Length,
    Has {
        index: Option<isize>,
        ident: Option<&'a str>,
    },
}
