//! Modeled after jq.  
//! The input to a filter is an array of JSON Values.
//! The filter is processed on each Value.  The results are
//! collected into a new array of values.  Thus, filters can be
//! chained.
//!
//! It is tempting to consider each input/output array to be references to
//! the original input.  This isn't attempted because a filter can transform
//! the inputs.

pub use action::*;
pub use block::*;
pub use errors::*;
pub use function::*;
pub use index_type::*;
pub use jq_peg::*;
pub use query::*;
pub use range_type::*;
pub use serde_json;
pub use token::*;

pub use serde_json::Value;

pub mod action;
pub mod block;
pub mod errors;
pub mod function;
pub mod index_type;
pub mod jq_peg;
pub mod query;
pub mod range_type;
pub mod token;

pub fn jq(json: &[u8], query_str: &str) -> Result<Vec<Value>, JQError> {
    let value: Value = serde_json::from_slice(json)?;
    let blocks = parse(query_str)?;
    query(&[value], blocks)
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::{IndexType, RangeType};
    use serde_json::json;

    #[test]
    fn test_hello_world() {
        let json = r#""Hello World!""#.as_bytes();
        let query_str = r#"."#;

        let result = jq(json, query_str).expect("Failed JQ");
        //dbg!(&result);
        assert_eq!(&result, &[json!("Hello World!")]);
    }

    #[test]
    fn test_object_identifier_index() {
        let json = r#"{"foo": 42, "bar": "less interesting data"}"#.as_bytes();
        let query_str = r#".foo"#;

        let result = jq(json, query_str).expect("Failed JQ");
        //dbg!(&result);
        assert_eq!(&result, &[json!(42)]);
    }

    #[test]
    fn test_bad_object_identifier_index() {
        let json = r#"{"notfoo": true, "alsonotfoo": false}"#.as_bytes();
        let query_str = r#".foo"#;

        let result = jq(json, query_str).expect("Failed JQ");
        //dbg!(&result);
        assert_eq!(&result, &[json!(null)]);
    }

    #[test]
    fn test_generic_object_index() {
        let json = r#"{"foo": 42}"#.as_bytes();
        let query_str = r#".["foo"]"#;

        let result = jq(json, query_str).expect("Failed JQ");
        //dbg!(&result);
        assert_eq!(&result, &[json!(42)]);
    }
    #[test]
    fn test_array_index() {
        let json = r#"[{"name":"JSON", "good":true}, {"name":"XML", "good":false}]"#.as_bytes();
        let query_str = ".[0]";

        let result = jq(json, query_str).expect("Failed JQ");
        //dbg!(&result);
        assert_eq!(
            &result,
            &[json!(
               {
                "name": "JSON",
                "good": true
               }
            )]
        );
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
