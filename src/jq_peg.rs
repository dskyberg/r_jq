use crate::{Block, Command, JQError, KeyType, Range, Token};

peg::parser!( grammar query_parser() for str {
    rule _ = [' ' | '\t']*

    /// Decimal number
    pub rule number() -> isize = n:$(['+' | '-']? ("0" / [ '0'..='9']+)) {n.parse().unwrap()}

    /// An identifier
    /// Must strt with an alpha character.  Can contain alphanumeric and '_'
    pub rule ident() -> Token<'input>
        = s:$(['a'..='z' | 'A'..='Z'] ['a'..='z' | 'A'..='Z' | '0'..='9' | '_' ]*) {Token::Ident(s)}


    pub rule string() -> &'input str
        = "\"" !"\"" s:$(['a'..='z' | 'A'..='Z' | '0'..='9' | ' ' | '_' | '-' | '/' | '#']*) "\"" {s}


    /// Range specifier: <start>:<end>
    /// Defines a start and end iterator
    /// Either value may be negative, in which case it counts backwards from
    /// the end of the array (for the start position) or from the start postion 
    /// (for the end position). Both values can be omitted, in which case it refers to the
    /// start or end of the array respectively.
    pub rule range() -> Range
        = precedence!{
            start:number() _ ":" _ end:number()
         { Range{start:Some(start), end:Some(end)} }
        --
            start:number() _ ":"? {Range::from_start(start)}
        --
            ":" _ end:number() {Range::from_end(end)}
        }

    /// An iterator iterates on an array.  It can be specified as:
    /// [] : iterates across the entire range
    /// For objects, the iterator may be a raw string element name.
    /// In the normal identifier,`.ident`, where `ident` contains special characters.
    /// ["name with special characters"]
    /// For arrays, the iterator may be an index or range
    /// [idx] : iterates on a single entry
    /// [range] : iterates across the defined range.  See [Range].
    pub rule iterator() -> Token<'input>
         = "[" _ r:range()? _ "]" { Token::Iterator(r)}


    pub rule identifier() -> Token<'input>
        = i:ident() / "[" _ i:string() _ "]" {Token::Ident(i)}

    /// A Key :
    /// '.' _ identifier()? _ iterator()?
    /// <element> - and element in the path
    /// <element>[specifier] - either a number (for array start) or an element in quotes
    /// Examples
    /// elem1
    /// array[<idx>]/
    /// obj["element name"]
    pub rule key() -> Token<'input>
        = "." _ i:identifier()? _ ii:iterator()? {?
            let identifier = match i {
                Some(Token::Ident(s)) => Some(s),
                _ => None,
            };
            let range = match ii {
                Some(Token::Iterator(Some(r)) ) => Some(r),
                Some(Token::Iterator(range)) => Some(Range::new()),
                _ => None,
            };
            Ok(Token::Key (KeyType{
                identifier,
                range,
            }))
        }

    /// A filter is a path of Keys
    pub rule filter() -> Token<'input>
        = _ k:key()+ _ {Token::Filter(k)}

    pub rule filters() -> Vec<Token<'input>>
        =  filter() ++  ","

    pub rule length() -> Command<'input>
        = _ "length" _ {Command::Length}

    pub rule has() -> Command<'input>
        = precedence!{
           _ "has(" _ ident:string()? _ ")" _ { Command::Has{index:None, ident}}
            --
           _ "has(" _ index:number()? _ ")" _ { Command::Has{index, ident:None}}
        }

    pub rule command() -> Command<'input>
        = l:length() / h:has()

    /// A block is either a set of filters or a command
    pub rule block() -> Block<'input>
        = _ filters:filters() _ {
            Block{filters: Some(filters)}
        }

    pub rule blocks() -> Vec<Block<'input>>
        = block() ** "|"
});

/// Parse query string
/// This ues the PEG grammer based [query_parser] above.  Since [query_parser] is not
/// callable outside the module, this method is provided for public access.
pub fn parse(input: &str) -> Result<Vec<Block>, JQError> {
    query_parser::blocks(input).map_err(|_| JQError::ParseError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blocks() {
        assert!(query_parser::blocks(r#" . , .b | ., .b"#).is_ok());
        let key1 = Token::Key(KeyType {
            identifier: None,
            range: None,
        });

        let key2 = Token::Key(KeyType {
            identifier: Some("b"),
            range: None,
        });

        let filter1 = Token::Filter(vec![key1]);
        let filter2 = Token::Filter(vec![key2]);
        let filters1 = vec![filter1, filter2];
        let block1 = Block {
            filters: Some(filters1),
        };

        let key3 = Token::Key(KeyType {
            identifier: None,
            range: None,
        });

        let key4 = Token::Key(KeyType {
            identifier: Some("b"),
            range: None,
        });

        let filter3 = Token::Filter(vec![key3]);
        let filter4 = Token::Filter(vec![key4]);
        let filters2 = vec![filter3, filter4];
        let block2 = Block {
            filters: Some(filters2),
        };
        let blocks = vec![block1, block2];
        assert_eq!(query_parser::blocks(r#" . , .b | ., .b"#), Ok(blocks));
    }

    #[test]
    fn test_block() {
        let key1 = Token::Key(KeyType {
            identifier: None,
            range: None,
        });

        let key2 = Token::Key(KeyType {
            identifier: Some("b"),
            range: None,
        });

        let filter1 = Token::Filter(vec![key1]);
        let filter2 = Token::Filter(vec![key2]);
        let filters = vec![filter1, filter2];
        let block = Block {
            filters: Some(filters),
        };

        assert_eq!(query_parser::block(r#"., .b"#), Ok(block));
    }

    #[test]
    fn test_command() {
        assert_eq!(query_parser::command("length"), Ok(Command::Length));

        assert_eq!(
            query_parser::command("has(\"abc\")"),
            Ok(Command::Has {
                ident: Some("abc"),
                index: None
            })
        );
    }

    #[test]
    fn test_has() {
        assert_eq!(
            query_parser::has(r#" has("some_path")"#),
            Ok(Command::Has {
                ident: Some("some_path"),
                index: None
            })
        );
        assert_eq!(
            query_parser::has(r#" has(2)"#),
            Ok(Command::Has {
                ident: None,
                index: Some(2)
            })
        );
    }

    #[test]
    fn test_length() {
        assert_eq!(query_parser::length(" length "), Ok(Command::Length));
    }

    #[test]
    fn test_filters() {
        assert_eq!(
            query_parser::filters("."),
            Ok(vec![Token::Filter(vec![Token::Key(KeyType {
                identifier: None,
                range: None,
            })])])
        );

        assert_eq!(
            query_parser::filters(". , .b"),
            Ok(vec![
                Token::Filter(vec![Token::Key(KeyType {
                    identifier: None,
                    range: None,
                })]),
                Token::Filter(vec![Token::Key(KeyType {
                    identifier: Some("b"),
                    range: None,
                })])
            ])
        );
    }

    #[test]
    fn test_filter() {
        assert_eq!(
            query_parser::filter("."),
            Ok(Token::Filter(vec![Token::Key(KeyType {
                identifier: None,
                range: None,
            })]))
        );

        assert_eq!(
            query_parser::filter(".a"),
            Ok(Token::Filter(vec![Token::Key(KeyType {
                identifier: Some("a"),
                range: None,
            })]))
        );

        assert_eq!(
            query_parser::filter(".a.b"),
            Ok(Token::Filter(vec![
                Token::Key(KeyType {
                    identifier: Some("a"),
                    range: None,
                }),
                Token::Key(KeyType {
                    identifier: Some("b"),
                    range: None,
                })
            ]))
        );

        assert_eq!(
            query_parser::filter(r#".["a"].b"#),
            Ok(Token::Filter(vec![
                Token::Key(KeyType {
                    identifier: Some("a"),
                    range: None,
                }),
                Token::Key(KeyType {
                    identifier: Some("b"),
                    range: None,
                })
            ]))
        );
    }

    #[test]
    fn test_key() {
        assert_eq!(
            query_parser::key("."),
            Ok(Token::Key(KeyType {
                identifier: None,
                range: None
            }))
        );
        assert_eq!(
            query_parser::key(". []"),
            Ok(Token::Key(KeyType {
                identifier: None,
                range: Some(Range {
                    start: None,
                    end: None
                })
            }))
        );

        assert_eq!(
            query_parser::key(".a"),
            Ok(Token::Key(KeyType {
                identifier: Some("a"),
                range: None
            }))
        );
        assert_eq!(
            query_parser::key(r#".["a"][]"#),
            Ok(Token::Key(KeyType {
                identifier: Some("a"),
                range: Some(Range {
                    start: None,
                    end: None
                })
            }))
        );

        assert_eq!(
            query_parser::key(r#".a[1:2]"#),
            Ok(Token::Key(KeyType {
                identifier: Some("a"),
                range: Some(Range {
                    start: Some(1),
                    end: Some(2)
                })
            }))
        );
        assert!(query_parser::key("some_segment.").is_err());
        assert!(query_parser::key(".some_segment.some_other_segment").is_err());
        assert!(query_parser::key("..").is_err());
    }

    #[test]
    fn test_iterator() {
        // Empty iterator
        assert_eq!(query_parser::iterator("[]"), Ok(Token::Iterator(None)));

        assert_eq!(query_parser::iterator("[ ]"), Ok(Token::Iterator(None)));

        // Index based iterator
        assert_eq!(
            query_parser::iterator("[2]"),
            Ok(Token::Iterator(Some(Range {
                start: Some(2),
                end: None
            })))
        );

        assert_eq!(
            query_parser::iterator("[ 2]"),
            Ok(Token::Iterator(Some(Range {
                start: Some(2),
                end: None
            })))
        );
        assert_eq!(
            query_parser::iterator("[2 ]"),
            Ok(Token::Iterator(Some(Range {
                start: Some(2),
                end: None
            })))
        );
        // Range based iterator
        assert_eq!(
            query_parser::iterator("[2:3]"),
            Ok(Token::Iterator(Some(Range {
                start: Some(2),
                end: Some(3)
            })))
        );
        assert_eq!(
            query_parser::iterator("[ 2:3 ]"),
            Ok(Token::Iterator(Some(Range {
                start: Some(2),
                end: Some(3)
            })))
        );
        assert_eq!(
            query_parser::iterator("[2 : 3]"),
            Ok(Token::Iterator(Some(Range {
                start: Some(2),
                end: Some(3)
            })))
        );

        // Identities are not iterators
        assert!(query_parser::iterator(r#"["abc"]"#).is_err());
    }

    #[test]
    fn test_range() {
        assert!(query_parser::range("1").is_ok());

        assert_eq!(
            query_parser::range("1:"),
            Ok(Range {
                start: Some(1),
                end: None
            })
        );
        assert_eq!(
            query_parser::range("1 :"),
            Ok(Range {
                start: Some(1),
                end: None
            })
        );

        assert_eq!(
            query_parser::range("-1:"),
            Ok(Range {
                start: Some(-1),
                end: None
            })
        );
        assert_eq!(
            query_parser::range(":1"),
            Ok(Range {
                start: None,
                end: Some(1)
            })
        );

        assert_eq!(
            query_parser::range(": 1"),
            Ok(Range {
                start: None,
                end: Some(1)
            })
        );
        assert_eq!(
            query_parser::range("1:2"),
            Ok(Range {
                start: Some(1),
                end: Some(2)
            })
        );
        assert_eq!(
            query_parser::range("1 : 2"),
            Ok(Range {
                start: Some(1),
                end: Some(2)
            })
        );
    }

    #[test]
    fn test_identifier() {
        assert_eq!(query_parser::identifier("Ab_1c"), Ok(Token::Ident("Ab_1c")));

        assert_eq!(
            query_parser::identifier(r#"["Ab 1c"]"#),
            Ok(Token::Ident("Ab 1c"))
        );
        assert!(query_parser::identifier(r#"[Ab_1c]"#).is_err());
    }

    #[test]
    fn test_ident() {
        assert_eq!(query_parser::ident("Ab_1c"), Ok(Token::Ident("Ab_1c")));
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
