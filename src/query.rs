use super::Value;

//use crate::Block;
use crate::{from_range, Block, JQError, Token};

/// Traverse an object
/// Single element lookup for an object value.  This is a nonterminal function.
/// The key must be an object identifier-index, so that a single value lookup is returned.  
/// This function can be called in a path traversal.
///
/// An error is returned if the value is not an object or the key is not an
/// object identifier-index.
pub fn query_object_element(object: &Value, key: &Token) -> Result<Value, JQError> {
    match key {
        Token::Ident(id) => Ok(object.get(id).unwrap_or(&Value::Null).clone()),
        Token::Index(idx) =>
        // If this is an identifier style index, return that
        {
            Ok(object
                .get(idx.as_identifier()?)
                .unwrap_or(&Value::Null)
                .clone())
        }
        _ => Err(JQError::ObjectQuery(format!("Wrong key type: {:?}", key))),
    }
}

/// Query an object with a Range token
/// If the key contains an empty range, then the value of each key is returned
/// as an array.  So `{"a":"a_val", "b":"b_val"}` is converted to `["a_val", "b_val"]
///
/// Returns Err if the element is not an object
pub fn query_object_range(value: &Value, key: &Token) -> Result<Vec<Value>, JQError> {
    let object = value.as_object().ok_or(JQError::NotAnObject)?;

    match key {
        Token::Range(range) => {
            if !range.is_empty() {
                return Err(JQError::ObjectQuery("Only empty range allowed".to_string()));
            }
            let mut values = Vec::new();
            // return each element in the object
            for (_, value) in object {
                values.push(value.clone())
            }
            Ok(values)
        }
        _ => Err(JQError::ObjectQuery(format!("Wrong key type: {:?}", key))),
    }
}

/// Travers an array.
///
/// Single element lookup for an array value.  This is a nonterminal function.
/// The key must be an index, so that a single value lookup is returned.
/// This function can be called in path traversal.
///
/// An error is returned if the value is not an array or the key is not an index.
pub fn query_array_element(value: &Value, key: &Token) -> Result<Value, JQError> {
    // Ensure the value is an array.  Convert for array style processing
    let array = value.as_array().ok_or(JQError::NotAnArray)?;

    match key {
        Token::Index(index) => {
            let mut idx = index.as_index()?;
            // If idx is negative, pull from end of array
            if idx < 0 {
                idx = array.len() as isize - idx;
            }
            Ok(array[idx as usize].clone())
        }
        _ => {
            dbg!(key);
            Err(JQError::ArrayQuery("Must be an index".to_string()))
        }
    }
}

/// Query an array.  This is a terminal query operation.
///
/// Returns an error if the value is not an array, or the key is not a range.
pub fn query_array_range(value: &Value, key: &Token) -> Result<Vec<Value>, JQError> {
    let array = value.as_array().ok_or(JQError::NotAnArray)?;

    match key {
        Token::Range(range) => {
            let mut values = Vec::new();
            if range.is_empty() {
                for val in array {
                    values.push(val.clone());
                }
            } else {
                // Convert the jq style range to a Rust range
                for val in from_range(array, range)? {
                    values.push(val.clone());
                }
            }
            Ok(values)
        }
        _ => Err(JQError::ArrayQuery(
            "Must be either an index or a range".to_string(),
        )),
    }
}

fn is_terminal(value: &Value, token: &Token) -> bool {
    match value {
        Value::Object(_) => matches!(token, Token::Range(_)),
        Value::Array(_) => matches!(token, Token::Range(_)),
        _ => true,
    }
}

/// Traverse a set of Token::Key values that represent a path.  Process
/// each input Value with the path.  
/// At each level, the path may represent either an object index or
/// an array iterator.  Thus the input to query_filter is always either
/// an object or an array
fn query_filter_single_value(input: &Value, filter: &Token) -> Result<Vec<Value>, JQError> {
    // If this filter is the identity filter, return the whole input
    if filter.is_identity() {
        return Ok(vec![input.clone()]);
    }

    let tokens = filter.as_filter()?;

    // The filter is a set of identifier-indexes that is
    // optionally terminated with a range.
    // The first step is to Walk the nonterminal identifier-indexes, until a
    // terminal token or the last key is reached.
    let mut token_idx = 0;
    let mut element = input.clone();

    while token_idx < tokens.len() {
        let token = &tokens[token_idx];
        token_idx += 1;

        // Make sure the element is not terminal
        if is_terminal(&element, token) {
            break;
        }

        element = match element {
            // If the input is an object, then either
            // query an element, or return the object
            Value::Object(_) => query_object_element(&element, token)?,
            Value::Array(_) => query_array_element(&element, token)?,
            _ => element,
        };
    }

    if token_idx == tokens.len() {
        // It looks like there is no range token to process.
        // So just vector up the result.
        Ok(vec![element])
    } else {
        let token = &tokens[token_idx];

        // It looks like the path concludes in a range token.
        match element {
            Value::Object(_) => query_object_range(&element, token),
            Value::Array(_) => query_array_range(&element, token),
            _ => Ok(vec![element]),
        }
    }
}

pub fn query_filter(inputs: &Vec<Value>, filter: &Token) -> Result<Vec<Value>, JQError> {
    let mut output: Vec<Value> = Vec::new();

    for input in inputs {
        let mut results = query_filter_single_value(input, filter)?;
        output.append(&mut results);
    }

    Ok(output)
}

/// Processes each filter, and returns the collected results
pub fn query_filters(in_values: &Vec<Value>, filters: Vec<Token>) -> Result<Vec<Value>, JQError> {
    let mut results: Vec<Value> = Vec::new();

    for filter in filters {
        let mut result = query_filter(in_values, &filter)?;
        results.append(&mut result);
    }

    Ok(results)
}

pub fn query_block(in_values: &Vec<Value>, block: Block) -> Result<Vec<Value>, JQError> {
    if let Some(filters) = &block.filters {
        return query_filters(in_values, filters.to_vec());
    }
    Ok(vec![Value::Null])
}

/// Queries a series of blocks.  The output of one block becomes the input for
/// the next block.
pub fn query(in_values: &[Value], blocks: Vec<Block>) -> Result<Vec<Value>, JQError> {
    let mut values = in_values.to_vec();

    for block in blocks {
        // The output from the last query_block is the input for the next
        values = query_block(&values, block)?;
    }
    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IndexType, RangeType};
    use serde_json::json;

    #[test]
    fn test_empty_query() {
        let json = include_str!("../test/basic.json");
        let input: Value = serde_json::from_str(json).expect("Failed to parse");

        let tokens = vec![];
        let filter = Token::Filter(tokens);
        let filters = vec![filter];
        let block = Block {
            filters: Some(filters),
        };
        let blocks = vec![block];
        let result = query(&[input.clone()], blocks).expect("Failed query");
        // dbg!(&result);
        assert_eq!(&result, &[input]);
    }

    #[test]
    fn test_query_2_blocks() {
        let json = include_str!("../test/basic.json");
        let input: Value = serde_json::from_str(json).expect("Failed to parse");

        //let query_str = r#" .object_1 | .elem_1 "#;
        let blocks = vec![
            Block {
                filters: Some(vec![Token::Filter(vec![Token::Ident("object_1")])]),
            },
            Block {
                filters: Some(vec![Token::Filter(vec![Token::Ident("elem_1")])]),
            },
        ];

        let result = query(&[input], blocks).expect("Failed query");

        //dbg!(&result);
        assert_eq!(&result, &[json!("Object 1 Element 1")]);
    }

    #[test]
    fn test_query() {
        let json = include_str!("../test/basic.json");
        let input: Value = serde_json::from_str(json).expect("Failed to parse");

        let tokens = vec![Token::Ident("object_1"), Token::Ident("elem_1")];
        let filter = Token::Filter(tokens);
        let filters = vec![filter];
        let block = Block {
            filters: Some(filters),
        };
        let blocks = vec![block];

        let result = query(&[input], blocks).expect("Failed query");
        //dbg!(&result);
        assert_eq!(&result, &[json!("Object 1 Element 1")]);
    }

    #[test]
    fn test_bock() {
        let json = include_str!("../test/basic.json");
        let input: Value = serde_json::from_str(json).expect("Failed to parse");

        let tokens = vec![Token::Ident("object_1"), Token::Ident("elem_1")];
        let filter = Token::Filter(tokens);
        let filters = vec![filter];
        let block = Block {
            filters: Some(filters),
        };

        let result = query_block(&vec![input], block).expect("Failed query");
        //dbg!(&result);
        assert_eq!(&result, &[json!("Object 1 Element 1")]);
    }

    #[test]
    fn test_2_filters() {
        let json = include_str!("../test/basic.json");
        let input: Value = serde_json::from_str(json).expect("Failed to parse");

        let filters = vec![
            Token::Filter(vec![Token::Ident("elem_1")]),
            Token::Filter(vec![Token::Ident("elem_2")]),
        ];

        let result = query_filters(&vec![input], filters).expect("Failed query");

        //dbg!(&result);
        assert_eq!(&result, &[json!("Element 1"), json!("Element 2")])
    }

    #[test]
    fn test_filters_object() {
        let json = include_str!("../test/basic.json");
        let input: Value = serde_json::from_str(json).expect("Failed to parse");

        let tokens = vec![Token::Ident("object_1"), Token::Ident("elem_1")];
        let filter = Token::Filter(tokens);
        let filters = vec![filter];

        let result = query_filters(&vec![input], filters).expect("Failed query");
        //dbg!(&result);
        assert_eq!(&result, &[json!("Object 1 Element 1")]);
    }

    #[test]
    fn test_filter_identity() {
        let json = include_bytes!("../test/basic.json");
        let input: Value = serde_json::from_slice(json).expect("Failed to parse json");

        let tokens = vec![Token::Identity];
        let filter = Token::Filter(tokens);
        let result = query_filter(&vec![input.clone()], &filter).expect("Failed query");

        //dbg!(&result);
        assert_eq!(&result, &vec![input]);
    }

    #[test]
    fn test_filter_object() {
        let json = include_str!("../test/basic.json");
        let input: Value = serde_json::from_str(json).expect("Failed to parse");

        let tokens = vec![Token::Ident("object_1"), Token::Ident("elem_1")];
        let filter = Token::Filter(tokens);

        let result = query_filter(&vec![input], &filter).expect("Failed query");
        assert_eq!(&result, &[json!("Object 1 Element 1")]);
    }

    #[test]
    fn test_filter_array() {
        let json = include_str!("../test/basic.json");
        let input: Value = serde_json::from_str(json).expect("Failed to parse");
        let tokens = vec![
            Token::Ident("array_1"),
            Token::Index(IndexType::from_index(2)),
        ];

        let filter = Token::Filter(tokens);

        let result = query_filter(&vec![input], &filter).expect("Failed query");
        assert_eq!(&result, &[json!("Array 1 Element 2")]);
    }

    #[test]
    fn test_query_object_by_ident() {
        let object = json!({"elem1":"element 1"});
        let key = Token::Ident("elem1");
        let result = query_object_element(&object, &key).expect("Failed to query");
        assert_eq!(result, json!("element 1"));
    }

    #[test]
    fn test_query_nested_object_by_ident() {
        let object = json!({"object_1":{"elem1":"element 1"}});
        let key = Token::Ident("object_1");
        let result = query_object_element(&object, &key).expect("query failed");

        assert_eq!(result, json!({"elem1":"element 1"}));
    }

    #[test]
    fn test_object_by_index() {
        let object = json!({"object_1":{"elem1":"element 1"}});
        let token = Token::Index(IndexType::from_identifier("object_1"));
        let result = query_object_element(&object, &token).expect("query failed");
        //dbg!(&result);
        assert_eq!(result, json!({"elem1":"element 1"}));
    }

    #[test]
    fn test_object_by_empty_index() {
        let object = json!({"a":"elem a", "b":"elem b"});
        let token = Token::Range(RangeType::new());
        let result = query_object_range(&object, &token).expect("query failed");
        //dbg!(&result);
        assert_eq!(result, vec![json!("elem a"), json!("elem b")]);
    }

    #[test]
    fn test_array_by_index() {
        let array = json!(["0", "1", "2"]);
        let token = Token::Index(IndexType::from_index(0));
        let result = query_array_element(&array, &token).expect("query failed");
        //dbg!(&result);
        assert_eq!(result, json!("0"));
    }

    #[test]
    fn test_array_by_empty_range() {
        let array = json!(["0", "1", "2"]);
        let token = Token::Range(RangeType::new());
        let result = query_array_range(&array, &token).expect("query failed");
        assert_eq!(result, vec![json!("0"), json!("1"), json!("2")]);
    }
}
