use super::Value;
use crate::{fn_has, fn_length, from_range, Action, Block, Filter, Function, JQError, Token};
use serde_json::Map;

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
fn query_object_range(object: &Map<String, Value>, key: &Token) -> Result<Vec<Value>, JQError> {
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
                idx += array.len() as isize;
            }
            // If idx is out of bounds, return null
            if idx >= array.len() as isize {
                return Ok(Value::Null);
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
fn query_array_range(array: &Vec<Value>, key: &Token) -> Result<Vec<Value>, JQError> {
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

/// Query an array.  This is a terminal query operation.
///
/// Returns an error if the value is not an array, or the key is not a range.
fn query_string_range(value: &str, key: &Token) -> Result<Vec<Value>, JQError> {
    match key {
        Token::Range(range) => {
            if range.is_empty() {
                return Ok(vec![Value::from(value)]);
            }
            let len = value.len();
            let (start, end) = range.as_slice(len);
            let val = value.get(start..end).unwrap_or("");

            Ok(vec![Value::from(val)])
        }
        _ => Err(JQError::ObjectQuery(format!("Wrong key type: {:?}", key))),
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
fn query_filter_single_value(input: &Value, tokens: &Filter) -> Result<Vec<Value>, JQError> {
    // The filter is a set of identifier-indexes that is
    // optionally terminated with a range.
    // The first step is to Walk the nonterminal identifier-indexes, until a
    // terminal token or the last key is reached.
    let mut token_idx = 0;
    let mut element = input.clone();

    while token_idx < tokens.len() {
        let token = &tokens[token_idx];
        token_idx += 1;

        // If this filter is the identity filter, return the whole input
        if token.is_identity() {
            continue;
        }

        // Make sure the element is not terminal
        if is_terminal(&element, token) {
            let result = match element {
                Value::Object(object) => query_object_range(&object, token)?,
                Value::Array(array) => query_array_range(&array, token)?,
                Value::String(s) => query_string_range(&s, token)?,
                _ => vec![element],
            };
            return Ok(result);
        }

        element = match element {
            // If the input is an object, then either
            // query an element, or return the object
            Value::Object(_) => query_object_element(&element, token)?,
            Value::Array(_) => query_array_element(&element, token)?,
            _ => element,
        };
    }

    Ok(vec![element])
}

fn query_filter(inputs: &Vec<Value>, filter: &Filter) -> Result<Vec<Value>, JQError> {
    let mut output: Vec<Value> = Vec::new();

    for input in inputs {
        let mut results = query_filter_single_value(input, filter)?;
        output.append(&mut results);
    }

    Ok(output)
}

fn query_function(inputs: &Vec<Value>, func: Function) -> Result<Vec<Value>, JQError> {
    let mut output: Vec<Value> = Vec::new();

    let mut results = match func {
        Function::Length => fn_length(inputs)?,
        Function::Has(has) => fn_has(inputs, &has)?,
    };
    output.append(&mut results);

    Ok(output)
}

/// Process all the actions in a block and return the results
fn query_block(in_values: &Vec<Value>, block: Block) -> Result<Vec<Value>, JQError> {
    let mut results: Vec<Value> = Vec::new();
    if block.actions.is_none() {
        return Ok(results);
    }
    for action in block.actions.unwrap() {
        let mut next = match action {
            Action::Filter(filter) => query_filter(in_values, &filter)?,
            Action::Function(func) => query_function(in_values, func)?,
        };
        results.append(&mut next);
    }
    Ok(results)
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
    use crate::{parse, IndexType, RangeType};
    use serde_json::json;

    #[test]
    fn test_array_filter() {
        let json = r#"[[1,2], "string", {"a":2}, null]"#;
        let input: Value = serde_json::from_str(json).expect("Failed to parse json");
        let blocks = parse(r#".[] | length"#).expect("failed to parse query");
        let result = query(&[input], blocks).expect("failed");
        // dbg!(&result);
        assert_eq!(
            result,
            [
                Value::from(2),
                Value::from(6),
                Value::from(1),
                Value::from(0)
            ]
        );
    }

    #[test]
    fn test_empty_query() {
        let json = include_str!("../test/basic.json");
        let input: Value = serde_json::from_str(json).expect("Failed to parse");

        let filter = vec![];
        let action = Action::Filter(filter);
        let block = Block {
            actions: Some(vec![action]),
        };
        let blocks = vec![block];
        let result = query(&[input.clone()], blocks).expect("Failed query");
        // dbg!(&result);
        assert_eq!(&result, &[input]);
    }

    #[test]
    fn test_query() {
        let json = include_str!("../test/basic.json");
        let input: Value = serde_json::from_str(json).expect("Failed to parse");

        let filter = vec![Token::Ident("object_1"), Token::Ident("elem_1")];
        let action = Action::Filter(filter);
        let block = Block {
            actions: Some(vec![action]),
        };
        let blocks = vec![block];

        let result = query(&[input], blocks).expect("Failed query");
        //dbg!(&result);
        assert_eq!(&result, &[json!("Object 1 Element 1")]);
    }

    #[test]
    fn test_query_2_blocks() {
        let json = include_str!("../test/basic.json");
        let input: Value = serde_json::from_str(json).expect("Failed to parse");

        //let query_str = r#" .object_1 | .elem_1 "#;
        let blocks = vec![
            Block {
                actions: Some(vec![Action::Filter(vec![Token::Ident("object_1")])]),
            },
            Block {
                actions: Some(vec![Action::Filter(vec![Token::Ident("elem_1")])]),
            },
        ];

        let result = query(&[input], blocks).expect("Failed query");

        //dbg!(&result);
        assert_eq!(&result, &[json!("Object 1 Element 1")]);
    }

    #[test]
    fn test_bock() {
        let json = include_str!("../test/basic.json");
        let input: Value = serde_json::from_str(json).expect("Failed to parse");

        let filter = vec![Token::Ident("object_1"), Token::Ident("elem_1")];
        let action = Action::Filter(filter);
        let block = Block {
            actions: Some(vec![action]),
        };

        let result = query_block(&vec![input], block).expect("Failed query");
        //dbg!(&result);
        assert_eq!(&result, &[json!("Object 1 Element 1")]);
    }

    #[test]
    fn test_filter_identity() {
        let json = include_bytes!("../test/basic.json");
        let input: Value = serde_json::from_slice(json).expect("Failed to parse json");

        let filter = vec![Token::Identity];
        let result = query_filter(&vec![input.clone()], &filter).expect("Failed query");

        //dbg!(&result);
        assert_eq!(&result, &vec![input]);
    }

    #[test]
    fn test_filter_object_with_identity() {
        let json = include_str!("../test/basic.json");
        let input: Value = serde_json::from_str(json).expect("Failed to parse");

        let filter = vec![
            Token::Identity,
            Token::Ident("object_1"),
            Token::Ident("elem_1"),
        ];

        let result = query_filter(&vec![input], &filter).expect("Failed query");
        assert_eq!(&result, &[json!("Object 1 Element 1")]);
    }

    #[test]
    fn test_filter_object() {
        let json = include_str!("../test/basic.json");
        let input: Value = serde_json::from_str(json).expect("Failed to parse");

        let filter = vec![Token::Ident("object_1"), Token::Ident("elem_1")];

        let result = query_filter(&vec![input], &filter).expect("Failed query");
        assert_eq!(&result, &[json!("Object 1 Element 1")]);
    }

    #[test]
    fn test_filter_array_with_identity() {
        let input = json!([{"name":"JSON", "good":true}, {"name":"XML", "good":false}]);

        let filter = vec![Token::Identity, Token::Index(IndexType::from(0))];

        let result = query_filter(&vec![input], &filter).expect("Failed query");
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
    fn test_filter_array() {
        let input = json!([{"name":"JSON", "good":true}, {"name":"XML", "good":false}]);

        let filter = vec![Token::Index(IndexType::from(0))];

        let result = query_filter(&vec![input], &filter).expect("Failed query");
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
        let token = Token::Index(IndexType::from("object_1"));
        let result = query_object_element(&object, &token).expect("query failed");
        //dbg!(&result);
        assert_eq!(result, json!({"elem1":"element 1"}));
    }

    #[test]
    fn test_object_by_empty_index() {
        let object = json!({"a":"elem a", "b":"elem b"});
        let token = Token::Range(RangeType::new());
        let result = query_object_range(object.as_object().unwrap(), &token).expect("query failed");
        //dbg!(&result);
        assert_eq!(result, vec![json!("elem a"), json!("elem b")]);
    }

    #[test]
    fn test_array_by_index() {
        let array = json!(["0", "1", "2"]);
        let token = Token::Index(IndexType::from(0));
        let result = query_array_element(&array, &token).expect("query failed");
        //dbg!(&result);
        assert_eq!(result, json!("0"));
    }

    #[test]
    fn test_array_negative_index() {
        let array = json!(["0", "1", "2"]);
        let token = Token::Index(IndexType::from(-2));
        let result = query_array_element(&array, &token).expect("query failed");
        //dbg!(&result);
        assert_eq!(result, json!("1"));
    }

    #[test]
    fn test_array_by_index_oob() {
        let array = json!(["0", "1", "2"]);
        let token = Token::Index(IndexType::from(3));
        let result = query_array_element(&array, &token).expect("query failed");
        //dbg!(&result);
        assert_eq!(result, json!(null));
    }

    #[test]
    fn test_array_by_empty_range() {
        let array = json!(["0", "1", "2"]);
        let token = Token::Range(RangeType::new());
        let result = query_array_range(array.as_array().unwrap(), &token).expect("query failed");
        assert_eq!(result, vec![json!("0"), json!("1"), json!("2")]);
    }
}
