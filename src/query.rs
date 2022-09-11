use serde_json::Value;

//use crate::Block;
use crate::{from_range, parse, Block, JQError, KeyType, Token};

/// If the key contains an identifier, then the result of querying that key
/// If the key contains an iterator, an error i thrown.
/// is returned.
/// Returns Err if the element is not an object
pub fn query_object(object: &Value, key: &KeyType) -> Result<Option<Value>, JQError> {
    // The Identity keytype is not allowed here.
    if !key.is_valid() {
        println!("***** ----- ERROR BAD KEY ------- ******");
        return Err(JQError::MalformedKeyType);
    }

    // For objects, only an empty range is allowed.
    if let Some(range) = &key.range {
        if key.identifier.is_some() {
            // This should never happen
            return Err(JQError::BadKeyType);
        }

        // An empty range is allowed, and just returns the object
        if range.start.is_none() && range.end.is_none() {
            return Ok(Some(object.clone()));
        }
        // This is an error.  Can't run a range on an object
        return Err(JQError::UnsupportedRange);
    }

    // If we're hear, there must be an identifier
    let id = key.identifier.unwrap();
    Ok(object.get(id).cloned())
}

/// If the key contains
/// Returns Err if the element is not an object
pub fn query_array(array: &Value, key: &KeyType) -> Result<Option<Value>, JQError> {
    if key.identifier.is_some() {
        return Err(JQError::UnsupportedObjectIndex);
    }
    match array {
        Value::Array(ary) => {
            // If there's no range, return the whole array
            match &key.range {
                Some(range) => {
                    let result = from_range(ary, range)?;
                    Ok(Some(Value::Array(result)))
                }
                _ => Ok(Some(array.clone())),
            }
        }
        _ => Err(JQError::NotAnArray),
    }
}

/// Traverse a set of Token::Key values
/// tokens represents a path to be descended.  
/// At each level, the path may represent either an object index or
/// an array iterator.  Thus the input to query_filter is always either
/// an object or an array
pub fn query_filter(input: &Value, filter: &Token) -> Result<Option<Value>, JQError> {
    // If this is just the identity filter, return input
    if filter.is_identity()? {
        return Ok(Some(input.clone()));
    }

    let tokens = filter.as_filter()?;

    // If this is being caled recursively, and
    // there are no more path segments, just return now.
    if tokens.is_empty() {
        return Ok(None);
    }

    let mut result: Option<Value> = Some(input.to_owned());
    for token in tokens {
        if result.is_none() {
            break;
        }
        let key = token.as_key()?;

        let layer = result.unwrap();
        result = match layer {
            // If the input is an object, then either
            // query an element, or return the object
            Value::Object(_) => query_object(&layer, key)?,
            Value::Array(_) => query_array(&layer, key)?,
            _ => None,
        };
    }
    Ok(result)
}

/// Processes each filter, and returns the collected results
pub fn query_filters(
    in_values: &Vec<Value>,
    filters: Vec<Token>,
) -> Result<Option<Vec<Value>>, JQError> {
    let mut values: Vec<Value> = Vec::new();

    for filter in filters {
        for value in in_values {
            if let Some(result) = query_filter(value, &filter)? {
                values.push(result);
            }
        }
    }
    if values.is_empty() {
        return Ok(None);
    }
    Ok(Some(values))
}

pub fn query_block(in_values: &Vec<Value>, block: Block) -> Result<Option<Vec<Value>>, JQError> {
    if let Some(filters) = &block.filters {
        return query_filters(in_values, filters.to_vec());
    }
    Ok(None)
}
pub fn query(in_value: &Value, blocks: Vec<Block>) -> Result<Option<Vec<Value>>, JQError> {
    let mut values = vec![in_value.to_owned()];
    for block in blocks {
        // The output from the last query_block is the input for the next
        let result = query_block(&values, block)?;
        values = match result {
            Some(vals) => vals,
            _ => vec![Value::Null],
        };
    }
    Ok(Some(values))
}

pub fn jq(value: &Value, query_str: &str) -> Result<Option<Vec<Value>>, JQError> {
    let blocks = parse(query_str)?;
    if blocks.is_empty() {
        println!("No blocks");
        return Ok(None);
    }
    println!("jq: {:?}", &blocks);

    query(value, blocks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Range;

    #[test]
    fn test_2_blocks() {
        let json = include_str!("../test/basic.json");
        let input = serde_json::from_str(json).expect("Failed to parse");

        let query_str = r#" .object_1 | .elem_1 "#;
        let result = jq(&input, query_str).expect("Failed JQ");

        dbg!(&result);
    }

    #[test]
    fn test_2_filters() {
        let json = include_str!("../test/basic.json");
        let input = serde_json::from_str(json).expect("Failed to parse");

        let query_str = r#" .elem_1, .elem_7 "#;
        let result = jq(&input, query_str).expect("Failed JQ");

        dbg!(&result);
    }

    #[test]
    fn test_empty_query() {
        let json = include_str!("../test/basic.json");
        let input = serde_json::from_str(json).expect("Failed to parse");

        let query_str = "";
        let result = jq(&input, query_str).expect("Failed JQ");

        assert!(result.is_none());
    }

    #[test]
    fn test_identity() {
        let json = include_str!("../test/basic.json");
        let input = serde_json::from_str(json).expect("Failed to parse");

        let query_str = ".";
        let result = jq(&input, query_str).expect("Failed JQ");
        assert_eq!(&result, &Some(vec![input]));
    }

    #[test]
    fn test_query() {
        let json = include_str!("../test/basic.json");
        let input = serde_json::from_str(json).expect("Failed to parse");

        let tokens: Vec<Token> = vec![
            Token::Key(KeyType::from_identifier("object_1")),
            Token::Key(KeyType::from_identifier("elem_1")),
        ];
        let filter = Token::Filter(tokens);
        let filters = vec![filter];
        let block = Block {
            filters: Some(filters),
        };
        let blocks = vec![block];

        let result = query(&input, blocks).expect("Failed query");
        dbg!(&result);
    }

    #[test]
    fn test_bock() {
        let json = include_str!("../test/basic.json");
        let input = serde_json::from_str(json).expect("Failed to parse");

        let tokens: Vec<Token> = vec![
            Token::Key(KeyType::from_identifier("object_1")),
            Token::Key(KeyType::from_identifier("elem_1")),
        ];
        let filter = Token::Filter(tokens);
        let filters = vec![filter];
        let block = Block {
            filters: Some(filters),
        };

        let result = query_block(&vec![input], block).expect("Failed query");
        dbg!(&result);
    }
    #[test]
    fn test_filters_object() {
        let json = include_str!("../test/basic.json");
        let input = serde_json::from_str(json).expect("Failed to parse");

        let tokens: Vec<Token> = vec![
            Token::Key(KeyType::from_identifier("object_1")),
            Token::Key(KeyType::from_identifier("elem_1")),
        ];
        let filter = Token::Filter(tokens);
        let filters = vec![filter];

        let result = query_filters(&vec![input], filters).expect("Failed query");
        dbg!(&result);
    }

    #[test]
    fn test_filter_object() {
        let json = include_str!("../test/basic.json");
        let input = serde_json::from_str(json).expect("Failed to parse");

        let tokens: Vec<Token> = vec![
            Token::Key(KeyType::from_identifier("object_1")),
            Token::Key(KeyType::from_identifier("elem_1")),
        ];
        let filter = Token::Filter(tokens);

        let result = query_filter(&input, &filter).expect("Failed query");
        let cmp_result: Value =
            serde_json::from_str(r#""Object 1 Element 1""#).expect("Failed to parse cmp");
        assert_eq!(result, Some(cmp_result));
    }

    #[test]
    fn test_filter_array() {
        let json = include_str!("../test/basic.json");
        let input = serde_json::from_str(json).expect("Failed to parse");

        let key1 = KeyType {
            identifier: Some("array_1"),
            range: None,
        };

        let key2 = KeyType {
            identifier: None,
            range: Some(Range::from_start(2)),
        };

        let tokens: Vec<Token> = vec![Token::Key(key1), Token::Key(key2)];
        let filter = Token::Filter(tokens);

        let result = query_filter(&input, &filter).expect("Failed query");
        dbg!(&result);
    }

    #[test]
    fn test_query_object() {
        let object: Value =
            serde_json::from_str(r#"{"elem1":"element 1"}"#).expect("Failed to parse");
        let key = KeyType::from_identifier("elem1");
        let cmp_result: Value =
            serde_json::from_str(r#""element 1""#).expect("Failed to parse cmp");
        let result = query_object(&object, &key).expect("query failed").unwrap();

        assert_eq!(&result, &cmp_result);
    }

    #[test]
    fn test_query_nested_object() {
        let object: Value =
            serde_json::from_str(r#"{"object_1":{"elem1":"element 1"}}"#).expect("Failed to parse");
        let key = KeyType::from_identifier("object_1");
        let cmp_result: Value =
            serde_json::from_str(r#"{"elem1":"element 1"}"#).expect("Failed to parse cmp");
        let result = query_object(&object, &key).expect("query failed").unwrap();

        assert_eq!(&result, &cmp_result);
    }

    #[test]
    fn test_object_with_range() {
        let object: Value =
            serde_json::from_str(r#"{"object_1":{"elem1":"element 1"}}"#).expect("Failed to parse");
        let key = KeyType::from_range(Range::new());
        let result = query_object(&object, &key).expect("query failed");
        dbg!(&result);
        //assert_eq!(&result, &object);
    }

    #[test]
    fn test_array() {
        let array: Value = serde_json::from_str(r#"["0", "1", "2"]"#).expect("Failed to parse");
        let key = KeyType::from_range(Range::new());
        let result = query_array(&array, &key).expect("query failed").unwrap();

        dbg!(&result);
    }
}
