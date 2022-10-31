#![doc = include_str!("../README.md")]
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

/// In case you are already dealiing with a serde_json::Value, use this
pub fn jq_from_value(value: &Value, query_str: &str) -> Result<Vec<Value>, JQError> {
    let blocks = parse(query_str)?;
    query(&[value.to_owned()], blocks)
}
#[cfg(test)]
mod tests {
    use super::*;

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
