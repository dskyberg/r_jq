/// PEG Parser
///
/// This module contains the PEG parser for parsing JQ query strings.
///
use crate::{Action, Block, Function, HasType, IndexType, JQError, RangeType, Token};

peg::parser!( grammar query_parser() for str {
    rule _ = [' ' | '\t']*

    /// Decimal number
    pub rule number() -> isize = n:$(['+' | '-']? ("0" / [ '0'..='9']+)) {n.parse().unwrap()}

    pub rule number_list() -> Vec<isize>
    =  number() ++  ","

    /// An identifier
    /// Must strt with an alpha character.  Can contain alphanumeric and '_'
    pub rule ident() -> Token<'input>
        = s:$(['a'..='z' | 'A'..='Z'] ['a'..='z' | 'A'..='Z' | '0'..='9' | '_' ]*) b:"?"? {Token::Ident(s, b.is_some())}


    pub rule string() -> &'input str
        = "\"" !"\"" s:$(['a'..='z' | 'A'..='Z' | '0'..='9' | ' ' | '_' | '-' | '/' | '#']*) "\"" {s}

    pub rule identity() -> Token<'input>
        = _ "." !"." _ { Token::Identity }

    /// A Range iterates on an object or array
    /// An empty range, `.[]`, iterates all values of an object or array
    /// If not enpty, a range has  `[start:end]` syntax.
    /// Either `start` or `end` may be omitted, but not both.
    pub rule range() -> Token<'input>
        = precedence!{
            _ "[" _ "]" _ {Token::Range(RangeType::new())}
            --
           _ "[" _ start:number() _ ":" _ "]" _ { Token::Range(RangeType::from_start(start)) }
           --
           _ "[" _ ":" _ end:number() _ "]" _ { Token::Range(RangeType::from_end(end)) }
           --
           _ "[" _ start:number() _  ":" _ end:number() _ "]" _ { Token::Range(RangeType::from_both(start,end)) }

        }

    /// An  index is either a object index: `[string]` or an array index: `[number]`
    /// Note: an empty set of brackets: `[]` is a range, not an index.
    pub rule index() -> Token<'input>
        = precedence! {
            _ "[" _ i:string() _ "]" _ {Token::Index(IndexType::from(i))}
            --
            _ "[" _ n:number_list() _ "]" _ {Token::Index(IndexType::from(n))}
        }

    pub rule identifier() -> Token<'input>
        = precedence! {
            _ "." _ i:ident() _ {i}
        --
            _ "." _ s:string() b:"?"? _  {Token::Ident(s, b.is_some())}
        }


    pub rule key() -> Token<'input>
        =  identifier() / index() / range() / identity()


    /// A filter is a path of Keys
    pub rule filter() -> Action<'input>
        = _ k:key()+ _ {Action::Filter(k)}

    pub rule length() -> Action<'input>
        = _ "length" _ {Action::Function(Function::Length)}

    pub rule has() -> Action<'input>
        = precedence!{
           _ "has(" _ ident:string() _ ")" _ { Action::Function(Function::Has(HasType::from(ident)))}
            --
           _ "has(" _ index:number() _ ")" _ { Action::Function(Function::Has(HasType::from(index)))}
        }

    pub rule recurse() -> Action<'input>
        = precedence!{
             _ ".." _ {Action::Function(Function::Recurse)}
            --
            _ "recurse" _ {Action::Function(Function::Recurse)}
        }

    pub rule keys() -> Action<'input>
    = _ "keys" f:"_unsorted"?_ { Action::Function(Function::Keys(f.is_none()))}


    pub rule function() -> Action<'input>
    = length() / has() / recurse() / keys()

    pub rule action() -> Action<'input>
        = filter() / function()

    pub rule actions() -> Vec<Action<'input>>
    =  action() ++  ","

    /// A block is either a set of filters or a command
    pub rule block() -> Block<'input>
        = _ actions:actions() _ {
            Block{actions: Some(actions)}
        }

    pub rule blocks() -> Vec<Block<'input>>
        = block() ** "|"
});

/// Parse query string
/// This ues the PEG grammer based `query_parser` above.  Since `query_parser` is not
/// callable outside the module, this method is provided for public access.
pub fn parse(input: &str) -> Result<Vec<Block>, JQError> {
    query_parser::blocks(input).map_err(|_| JQError::ParseError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recurse() {
        let query = parse(r#".[] | .."#).expect("Failed");
        dbg!(&query);
    }

    #[test]
    fn test_identity_to_array() {
        let query = query_parser::block(".[]");
        dbg!(&query);
    }

    #[test]
    fn test_blocks() {
        assert_eq!(
            query_parser::blocks(r#" . , .b | ., .b"#),
            Ok(vec![
                Block {
                    actions: Some(vec![
                        Action::Filter(vec![Token::Identity,],),
                        Action::Filter(vec![Token::Ident("b", false),],),
                    ],),
                },
                Block {
                    actions: Some(vec![
                        Action::Filter(vec![Token::Identity,],),
                        Action::Filter(vec![Token::Ident("b", false),],),
                    ],),
                },
            ])
        );
    }

    #[test]
    fn test_block_filter_and_function() {
        assert_eq!(
            query_parser::block("., length "),
            Ok(Block {
                actions: Some(vec![
                    Action::Filter(vec![Token::Identity]),
                    Action::Function(Function::Length)
                ])
            })
        );
    }

    #[test]
    fn test_block_1_function() {
        assert_eq!(
            query_parser::block(" length "),
            Ok(Block {
                actions: Some(vec![Action::Function(Function::Length)])
            })
        );
    }

    #[test]
    fn test_block_2_filters() {
        assert_eq!(
            query_parser::block(". , .b"),
            Ok(Block {
                actions: Some(vec![
                    Action::Filter(vec![Token::Identity]),
                    Action::Filter(vec![Token::Ident("b", false)])
                ])
            })
        );
    }

    #[test]
    fn test_block_1_filter() {
        assert_eq!(
            query_parser::block(".b"),
            Ok(Block {
                actions: Some(vec![Action::Filter(vec![Token::Ident("b", false)])])
            })
        );
    }

    #[test]
    fn test_function_has() {
        assert_eq!(
            query_parser::has(r#" has("some_path")"#),
            Ok(Action::Function(Function::Has(HasType::from("some_path"))))
        );
        assert_eq!(
            query_parser::has(r#" has(2)"#),
            Ok(Action::Function(Function::Has(HasType::from(2))))
        );
    }

    #[test]
    fn test_function_length() {
        assert_eq!(
            query_parser::length(" length "),
            Ok(Action::Function(Function::Length))
        );
    }

    #[test]
    fn test_filter() {
        assert_eq!(
            query_parser::filter("."),
            Ok(Action::Filter(vec![Token::Identity]))
        );

        assert_eq!(
            query_parser::filter(".a"),
            Ok(Action::Filter(vec![Token::Ident("a", false)]))
        );

        assert_eq!(
            query_parser::filter(".[]"),
            Ok(Action::Filter(vec![
                Token::Identity,
                Token::Range(RangeType::new())
            ]))
        );

        assert_eq!(
            query_parser::filter(".a.b"),
            Ok(Action::Filter(vec![
                Token::Ident("a", false),
                Token::Ident("b", false)
            ]))
        );

        assert_eq!(
            query_parser::filter(r#"."a".b"#),
            Ok(Action::Filter(vec![
                Token::Ident("a", false),
                Token::Ident("b", false)
            ]))
        );

        assert_eq!(
            query_parser::filter(r#".["a"].b"#),
            Ok(Action::Filter(vec![
                Token::Identity,
                Token::Index(IndexType::from("a")),
                Token::Ident("b", false)
            ]))
        );
    }

    #[test]
    fn test_key() {
        assert_eq!(query_parser::key("."), Ok(Token::Identity));

        assert_eq!(query_parser::key(".a"), Ok(Token::Ident("a", false)));

        assert_eq!(query_parser::key(r#"."a""#), Ok(Token::Ident("a", false)));

        assert_eq!(
            query_parser::key(r#"["a"]"#),
            Ok(Token::Index(IndexType::from("a")))
        );

        assert_eq!(
            query_parser::key(r#"[2]"#),
            Ok(Token::Index(IndexType::from(2)))
        );

        assert_eq!(query_parser::key("[]"), Ok(Token::Range(RangeType::new())));

        assert_eq!(
            query_parser::key(r#"[1:2]"#),
            Ok(Token::Range(RangeType::from_both(1, 2)))
        );
    }

    #[test]
    fn test_empty_index() {
        // Empty iterator
        assert!(query_parser::index("[]").is_err());
    }

    #[test]
    fn test_identifier_index() {
        assert_eq!(
            query_parser::index(r#"["a"]"#),
            Ok(Token::Index(IndexType::from("a")))
        );

        assert_eq!(
            query_parser::index(r#"[ "a" ]"#),
            Ok(Token::Index(IndexType::from("a")))
        );
    }

    #[test]
    fn test_index_index() {
        assert_eq!(
            query_parser::index("[2]"),
            Ok(Token::Index(IndexType::from(2)))
        );

        assert_eq!(
            query_parser::index("[ 2]"),
            Ok(Token::Index(IndexType::from(2)))
        );

        assert_eq!(
            query_parser::index("[2 ]"),
            Ok(Token::Index(IndexType::from(2)))
        );
    }

    #[test]
    fn test_negative_index() {
        assert_eq!(
            query_parser::index("[-2]"),
            Ok(Token::Index(IndexType::from(-2)))
        );
    }

    #[test]
    fn test_empty_range() {
        assert!(query_parser::range("[:]").is_err());
    }

    #[test]
    fn test_range_start_only() {
        assert_eq!(
            query_parser::range("[1:]"),
            Ok(Token::Range(RangeType::from_start(1)))
        );

        assert_eq!(
            query_parser::range("[1 :]"),
            Ok(Token::Range(RangeType::from_start(1)))
        );

        assert_eq!(
            query_parser::range("[-1:]"),
            Ok(Token::Range(RangeType::from_start(-1)))
        );
    }

    #[test]
    fn test_range_end_only() {
        assert_eq!(
            query_parser::range("[:1]"),
            Ok(Token::Range(RangeType::from_end(1)))
        );

        assert_eq!(
            query_parser::range("[: 1]"),
            Ok(Token::Range(RangeType::from_end(1)))
        );
    }

    #[test]
    fn test_range_start_end() {
        assert_eq!(
            query_parser::range("[1:2]"),
            Ok(Token::Range(RangeType::from_both(1, 2)))
        );

        assert_eq!(
            query_parser::range("[1 : 2]"),
            Ok(Token::Range(RangeType::from_both(1, 2)))
        );

        assert_eq!(
            query_parser::range("[ 1 : 2 ]"),
            Ok(Token::Range(RangeType::from_both(1, 2)))
        );

        assert_eq!(
            query_parser::range("[ 1 : 2 ]"),
            Ok(Token::Range(RangeType::from_both(1, 2)))
        );
    }

    #[test]
    fn test_identifier() {
        assert_eq!(
            query_parser::identifier(".Ab_1c"),
            Ok(Token::Ident("Ab_1c", false))
        );

        assert_eq!(
            query_parser::identifier(r#"."Ab 1c""#),
            Ok(Token::Ident("Ab 1c", false))
        );
    }

    #[test]
    fn test_ident() {
        assert_eq!(
            query_parser::ident("Ab_1c"),
            Ok(Token::Ident("Ab_1c", false))
        );
        assert!(query_parser::ident("1Ab_1c").is_err());
    }

    #[test]
    fn test_string() {
        assert_eq!(query_parser::string(r#""abc""#), Ok("abc"));
        assert_eq!(query_parser::string(r#""a 1_bc""#), Ok("a 1_bc"));
        assert_eq!(query_parser::string(r#"" a 1_bc ""#), Ok(" a 1_bc "));
    }

    #[test]
    fn test_numbers() {
        assert_eq!(query_parser::number("0"), Ok(0));
        assert!(query_parser::number("01").is_err());
        assert_eq!(query_parser::number("123"), Ok(123));
        assert_eq!(query_parser::number("+123"), Ok(123));
        assert_eq!(query_parser::number("-123"), Ok(-123));
        assert!(query_parser::number("+").is_err());
        assert!(query_parser::number("-").is_err());
        assert!(query_parser::number("123+").is_err());
        assert!(query_parser::number("123-").is_err());
    }
}
