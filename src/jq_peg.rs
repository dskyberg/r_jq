/// PEG Parser
///
/// This module contains the PEG parser for parsing JQ query strings.
///
use crate::{
    Action, Block, ExpressionType, Function, HasType, IndexType, JQError, Operator, RangeType,
    Token,
};

peg::parser!( grammar query_parser() for str {
    rule _ = [' ' | '\t']*

    /// floating point number4
    pub rule number() -> f64 = n:$(['+' | '-']? [ '0'..='9']+ ['.']? ['0'..='9']*) {n.parse().unwrap()}

    pub rule number_list() -> Vec<f64> =  number() ++  ","


    pub rule operator() -> Operator
        = s:$( ">=" / "<=" / "+" / "-" / "*" / "/" / "<" / ">" / "!=" / "==") {Operator::try_from(s).expect("failed")}

    rule sum_ops() -> Operator = s:$("+" / "-"/ "<" / ">" / "!=" / "==" / ">=" / "<=" )  {Operator::try_from(s).expect("failed")}
    rule mult_ops() -> Operator = s:$("*" / "/") {Operator::try_from(s).expect("failed")}

    pub rule expression() -> ExpressionType<'input>
        = sum()

    rule sum() -> ExpressionType<'input>
        = l:product() _ o:sum_ops() _ r:product() { ExpressionType::Op(o, Box::new(l), Box::new(r)) }
        / product()

    rule product() -> ExpressionType<'input>
        = l:atom() _ o:mult_ops() _ r:atom() { ExpressionType::Op(o, Box::new(l), Box::new(r)) }
        / atom()

    rule atom() -> ExpressionType<'input>
        = number_exp()
        / string_exp()
        / i:ident_exp()
        / "(" _ v:sum() _ ")" { v }

    rule ident_exp() ->  ExpressionType<'input> = k:key()    { ExpressionType::Ident(k)}
    rule string_exp() -> ExpressionType<'input> = s:string() { ExpressionType::String(s)}
    rule number_exp() -> ExpressionType<'input> = n:number() { ExpressionType::Number(n) }

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
           _ "[" _ start:number() _ ":" _ "]" _ { Token::Range(RangeType::from_start(start as isize)) }
           --
           _ "[" _ ":" _ end:number() _ "]" _ { Token::Range(RangeType::from_end(end as isize)) }
           --
           _ "[" _ start:number() _  ":" _ end:number() _ "]" _ { Token::Range(RangeType::from_both(start as isize,end as isize)) }

        }

    /// An  index is either a object index: `[string]` or an array index: `[number]`
    /// Note: an empty set of brackets: `[]` is a range, not an index.
    pub rule index() -> Token<'input>
        = precedence! {
            _ "[" _ i:string() _ "]" b:"?"?_ {Token::Index(IndexType::from((i, b.is_some())))}
            --
            _ "[" _ n:number_list() _ "]" b:"?"? _ {
                let list: Vec<isize> = n.iter().map(|f| *f as isize).collect();
                Token::Index(IndexType::from((list, b.is_some())))
            }
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
           _ "has(" _ index:number() _ ")" _ { Action::Function(Function::Has(HasType::from(index as isize)))}
        }

    pub rule select()
        = _ "select(" _ s1:string() _ o:operator() _  s2:string() _ ")" _

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

    pub rule expr() -> Action<'input>
    = e:expression() {Action::Expression(e)}

    pub rule action() -> Action<'input>
        = filter() / function() / expr()

    pub rule actions() -> Vec<Action<'input>>
    =  action() ++  ","

    pub rule collect() -> Block<'input>
        = _ "[" _ a:actions() _ "]" _ {
            Block{actions: Some(a), collect: true}
        }

    /// A block is either a set of filters or a command
    pub rule block() -> Block<'input>
        = precedence! {
            _ actions:actions() _ {Block{actions: Some(actions),collect: false}}
        --
            collect: collect() {collect}
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
    fn test_expresion() {
        let result = parse(r#"(.a + 2)"#);
        dbg!(&result);
    }

    #[test]
    fn test_recurse() {
        let result = parse(r#".[] | .."#).expect("Failed");
        let query = vec![
            Block {
                actions: Some(vec![Action::Filter(vec![
                    Token::Identity,
                    Token::Range(RangeType::new()),
                ])]),
                collect: false,
            },
            Block {
                actions: Some(vec![Action::Function(Function::Recurse)]),
                collect: false,
            },
        ];
        assert_eq!(result, query);
    }

    #[test]
    fn test_identity_to_array() {
        let query = query_parser::block(".[]");
        assert_eq!(
            query,
            Ok(Block {
                actions: Some(vec![Action::Filter(vec![
                    Token::Identity,
                    Token::Range(RangeType::new()),
                ])]),
                collect: false,
            })
        )
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
                    collect: false,
                },
                Block {
                    actions: Some(vec![
                        Action::Filter(vec![Token::Identity,],),
                        Action::Filter(vec![Token::Ident("b", false),],),
                    ],),
                    collect: false
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
                ]),
                collect: false
            })
        );
    }

    #[test]
    fn test_block_1_function() {
        assert_eq!(
            query_parser::block(" length "),
            Ok(Block {
                actions: Some(vec![Action::Function(Function::Length)]),
                collect: false,
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
                ]),
                collect: false
            },)
        );
    }

    #[test]
    fn test_block_collect() {
        //let collect = query_parser::collect("[ .[] ]");
        let result = parse("[.[]]").expect("fail");
        let collect = vec![Block {
            actions: Some(vec![Action::Filter(vec![
                Token::Identity,
                Token::Range(RangeType::new()),
            ])]),
            collect: true,
        }];
        assert_eq!(result, collect);
    }

    #[test]
    fn test_block_1_filter() {
        assert_eq!(
            query_parser::block(".b"),
            Ok(Block {
                actions: Some(vec![Action::Filter(vec![Token::Ident("b", false)])]),
                collect: false
            })
        );
    }

    #[test]
    fn test_select() {
        let result = query_parser::select(r#"select("a" != "a")"#);
        dbg!(&result);
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
                Token::Index(IndexType::from(("a", false))),
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
            Ok(Token::Index(IndexType::from(("a", false))))
        );

        assert_eq!(
            query_parser::key(r#"[2]"#),
            Ok(Token::Index(IndexType::from((2, false))))
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
            Ok(Token::Index(IndexType::from(("a", false))))
        );

        assert_eq!(
            query_parser::index(r#"[ "a" ]"#),
            Ok(Token::Index(IndexType::from(("a", false))))
        );
    }

    #[test]
    fn test_index_index() {
        assert_eq!(
            query_parser::index("[2]"),
            Ok(Token::Index(IndexType::from((2, false))))
        );

        assert_eq!(
            query_parser::index("[ 2]"),
            Ok(Token::Index(IndexType::from((2, false))))
        );

        assert_eq!(
            query_parser::index("[2 ]"),
            Ok(Token::Index(IndexType::from((2, false))))
        );
    }

    #[test]
    fn test_negative_index() {
        assert_eq!(
            query_parser::index("[-2]"),
            Ok(Token::Index(IndexType::from((-2, false))))
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
    fn test_operators() {
        assert_eq!(query_parser::operator("+"), Ok(Operator::Plus));
        assert_eq!(query_parser::operator("=="), Ok(Operator::Equal));
        assert_eq!(query_parser::operator("!="), Ok(Operator::NotEqual));
        assert_eq!(query_parser::operator(">="), Ok(Operator::Gte));
        assert_eq!(query_parser::operator("<="), Ok(Operator::Lte));
    }

    #[test]
    fn test_numbers() {
        assert_eq!(query_parser::number("0"), Ok(0.0));
        assert_eq!(query_parser::number("123"), Ok(123.0));
        assert_eq!(query_parser::number("+123"), Ok(123.0));
        assert_eq!(query_parser::number("-123"), Ok(-123.0));
        assert_eq!(query_parser::number("01"), Ok(01.0));
        assert_eq!(query_parser::number("01.01"), Ok(1.01));
        assert_eq!(query_parser::number("-01.01"), Ok(-1.01));
        assert!(query_parser::number("+").is_err());
        assert!(query_parser::number("-").is_err());
        assert!(query_parser::number("123+").is_err());
        assert!(query_parser::number("123-").is_err());
    }
}
