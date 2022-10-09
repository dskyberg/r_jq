//! Modeled after jq.
//! ## Overview
//! A query is a set of filters and functions that operate
//! on an collection of [Values](serde_json::Value) and produce
//! a new collection of [Values](serde_json::Value).
//!
//! It is tempting to consider each input/output array to be references to
//! the original input.  This isn't attempted because a block of filters and
//! actions can transform the orginal inputs.
//!
//! # Examples
//!
//! ## Identity
//!
//! ```rust
//! use r_jq::jq;
//! use serde_json::json;
//!
//! let json = r#""Hello World!""#.as_bytes();
//! let query_str = r#"."#;
//!
//! let result = jq(json, query_str).expect("Failed JQ");
//! assert_eq!(&result, &[json!("Hello World!")]);
//! ```
//!
//! ## Object Identifier-Index: `.foo`, `.foo.bar`
//!
//! ```rust
//! use r_jq::jq;
//! use serde_json::json;
//!
//! let json = r#"{"foo": 42, "bar": "less interesting data"}"#.as_bytes();
//! let query_str = r#".foo"#;
//!
//! let result = jq(json, query_str).expect("Failed JQ");
//! assert_eq!(&result, &[json!(42)]);
//! ```
//!
//! ```rust
//! use r_jq::jq;
//! use serde_json::json;
//!
//! let json = r#"{"notfoo": true, "alsonotfoo": false}"#.as_bytes();
//! let query_str = r#".foo"#;
//!
//! let result = jq(json, query_str).expect("Failed JQ");
//! assert_eq!(&result, &[json!(null)]);
//! ```
//!
//! ```rust
//! use r_jq::jq;
//! use serde_json::json;
//!
//! let json = r#"{"foo": 42}"#.as_bytes();
//! let query_str = r#".["foo"]"#;
//!
//! let result = jq(json, query_str).expect("Failed JQ");
//! assert_eq!(&result, &[json!(42)]);
//! ```
//!
//! ## Array Index: `.[2]`
//!
//! ```rust
//! use r_jq::jq;
//! use serde_json::json;
//!
//! let json = r#"[{"name":"JSON", "good":true}, {"name":"XML", "good":false}]"#.as_bytes();
//! let query_str = ".[0]";
//!
//! let result = jq(json, query_str).expect("Failed JQ");
//!
//! assert_eq!(&result, &[json!({"name": "JSON", "good": true})]);
//! ```
//!
//! ```rust
//! use r_jq::jq;
//! use serde_json::json;
//!
//! let json = r#"[{"name":"JSON", "good":true}, {"name":"XML", "good":false}]"#.as_bytes();
//! let query_str = r#".[0]"#;
//!
//! let result = jq(json, query_str).expect("Failed JQ");
//! assert_eq!(&result, &[json!({"name":"JSON", "good":true})]);
//! ```
//!
//! ```rust
//! use r_jq::jq;
//! use serde_json::json;
//!
//! let json = r#"[{"name":"JSON", "good":true}, {"name":"XML", "good":false}]"#.as_bytes();
//! let query_str = r#".[2]"#;
//!
//! let result = jq(json, query_str).expect("Failed JQ");
//! assert_eq!(&result, &[json!(null)]);
//! ```
//!
//! ```rust
//! use r_jq::jq;
//! use serde_json::json;
//!
//! let json = r#"[1,2,3]"#.as_bytes();
//! let query_str = r#".[-2]"#;
//!
//! let result = jq(json, query_str).expect("Failed JQ");
//! assert_eq!(&result, &[json!(2)]);
//! ```
//!
//! ## Array/String Slice: `.[10:15]`
//!
//! ```rust
//! use r_jq::jq;
//! use serde_json::json;
//!
//! let json = r#"["a","b","c","d","e"]"#.as_bytes();
//! let query_str = r#".[2:4]"#;
//!
//! let result = jq(json, query_str).expect("Failed JQ");
//! assert_eq!(&result, &[json!("c"), json!("d")]);
//! ```
//!
//! ```rust
//! use r_jq::jq;
//! use serde_json::json;
//!
//! let json = r#"["a","b","c","d","e"]"#.as_bytes();
//! let query_str = r#".[:3]"#;
//!
//! let result = jq(json, query_str).expect("Failed JQ");
//! assert_eq!(&result, &[json!("a"),json!("b"), json!("c")]);
//! ```
//!
//! ```rust
//! use r_jq::jq;
//! use serde_json::json;
//!
//! let json = r#""abcdefghi""#.as_bytes();
//! let query_str = r#".[2:4]"#;
//!
//! let result = jq(json, query_str).expect("Failed JQ");
//! assert_eq!(&result, &[json!("cd")]);
//! ```
//!
//! ```rust
//! use r_jq::jq;
//! use serde_json::json;
//!
//! let json = r#"["a","b","c","d","e"]"#.as_bytes();
//! let query_str = r#".[-2:]"#;
//!
//! let result = jq(json, query_str).expect("Failed JQ");
//! assert_eq!(&result, &[json!("d"),json!("e")]);
//! ```
//!
//! ## Array/Object Value Iterator: `.[]`
//!
//! ```rust
//! use r_jq::jq;
//! use serde_json::json;
//!
//! let json = r#"[{"name":"JSON", "good":true}, {"name":"XML", "good":false}]"#.as_bytes();
//! let query_str = r#".[]"#;
//!
//! let result = jq(json, query_str).expect("Failed JQ");
//! assert_eq!(&result, &[json!({"name":"JSON", "good":true}),json!({"name":"XML", "good":false})]);
//! ```
//!
//! ```rust
//! use r_jq::jq;
//! use serde_json::json;
//!
//! let json = r#"[]"#.as_bytes();
//! let query_str = r#".[]"#;
//!
//! let result = jq(json, query_str).expect("Failed JQ");
//! assert_eq!(&result, &Vec::<serde_json::Value>::new());
//! ```
//!
//! ```rust
//! use r_jq::jq;
//! use serde_json::json;
//!
//! let json = r#"{"a": 1, "b": 1}"#.as_bytes();
//! let query_str = r#".[]"#;
//!
//! let result = jq(json, query_str).expect("Failed JQ");
//! assert_eq!(&result, &[json!(1),json!(1)]);
//! ```
//!
//! ## Comma `.`
//!
//! ```rust
//! use r_jq::jq;
//! use serde_json::json;
//!
//! let json = r#"{"foo": 42, "bar": "something else", "baz": true}"#.as_bytes();
//! let query_str = r#".foo, .bar"#;
//!
//! let result = jq(json, query_str).expect("Failed JQ");
//! assert_eq!(&result, &[json!(42), json!("something else")]);
//! ```
//!
//! ```rust
//! use r_jq::jq;
//! use serde_json::json;
//!
//! let json = r#"{"user":"stedolan", "projects": ["jq", "wikiflow"]}"#.as_bytes();
//! let query_str = r#".user, .projects[]"#;
//!
//! let result = jq(json, query_str).expect("Failed JQ");
//! assert_eq!(&result, &[json!("stedolan"), json!("jq"),json!("wikiflow")]);
//! ```
//!
//! ## Pipe: `|`
//!
//! ```rust
//! use r_jq::jq;
//! use serde_json::json;
//!
//! let json = r#"[{"name":"JSON", "good":true}, {"name":"XML", "good":false}]"#.as_bytes();
//! let query_str = r#".[] | .name"#;
//!
//! let result = jq(json, query_str).expect("Failed JQ");
//! assert_eq!(&result, &[json!("JSON"), json!("XML")]);
//! ```
//!

#![warn(missing_docs)]
use action::*;
use block::*;
use errors::*;
use function::*;
use has_type::*;
use index_type::*;
use jq_peg::*;
use query::*;
use range_type::*;
pub use serde_json;
use token::*;

use serde_json::Value;
/// Contains Action
pub mod action;
/// Contains Block
pub mod block;
/// Contains JQError
pub mod errors;
/// Contains Function
pub mod function;
/// Contains HasType
pub mod has_type;
/// Contains IndexType
pub mod index_type;
#[doc(hidden)]
pub mod jq_peg;
/// Contains the query functions
pub mod query;
/// Contains RangeType
pub mod range_type;
/// Contains Token
pub mod token;

/// This is the function that users of the r_jq library will call.
pub fn jq(json: &[u8], query_str: &str) -> Result<Vec<Value>, JQError> {
    let value: Value = serde_json::from_slice(json)?;
    let blocks = parse(query_str)?;
    query(&[value], blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comma_in_range() {
        // TODO: fix this
        /*
                use serde_json::json;
                let json = r#"["a","b","c","d","e"]"#.as_bytes();
                let query_str = r#".[4,2]"#;

                let result = jq(json, query_str).expect("Failed JQ");
                assert_eq!(&result, &[json!("e"), json!("c")]);
        */
    }

    #[test]
    fn test_identity() {
        let json = include_bytes!("../test/basic.json");
        let input: Value = serde_json::from_slice(json).expect("Failed to parse json");

        let query_str = ".";
        let result = jq(json, query_str).expect("Failed JQ");
        //dbg!(&result);
        assert_eq!(&result, &[input]);
    }

    #[test]
    fn test_empty_query() {
        let json = include_bytes!("../test/basic.json");
        let input: Value = serde_json::from_slice(json).expect("Failed to parse json");

        let query_str = "";
        let result = jq(json, query_str).expect("Failed JQ");

        assert_eq!(&result, &[input]);
    }
}
