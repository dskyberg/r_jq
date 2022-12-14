use super::Value;
use crate::{
    fn_has, fn_keys, fn_length, fn_recurse, from_range, Action, Block, Filter, Function, IndexType,
    JQError, RangeType, Token,
};
use serde_json::Map;

fn value_name(value: &Value) -> &str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

/// Called by [fn_has]
pub fn query_object_ident(object: &Map<String, Value>, id: &str) -> Result<Vec<Value>, JQError> {
    Ok(vec![object.get(id).unwrap_or(&Value::Null).clone()])
}

/// Traverse an object
/// Single element lookup for an object value.  This is a nonterminal function.
/// The key must be an object identifier-index, so that a single value lookup is returned.  
/// This function can be called in a path traversal.
///
/// An error is returned if the value is not an object or the key is not an
/// object identifier-index.
pub fn query_object_index(
    object: &Map<String, Value>,
    idx: &IndexType,
) -> Result<Vec<Value>, JQError> {
    let (index, silent) = idx.as_identifier()?;
    let value = object.get(index);
    match value {
        Some(result) => Ok(vec![result.to_owned()]),
        None => {
            if silent {
                Ok(Vec::<Value>::new())
            } else {
                Err(JQError::ObjectQuery(format!(
                    "bad object index {:?}",
                    &index
                )))
            }
        }
    }
}

/// Query an object with a Range token
/// If the key contains an empty range, then the value of each key is returned
/// as an array.  So `{"a":"a_val", "b":"b_val"}` is converted to `["a_val", "b_val"]
///
/// Returns Err if the element is not an object
fn query_object_range(
    object: &Map<String, Value>,
    range: &RangeType,
) -> Result<Vec<Value>, JQError> {
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

/// Travers an array.
///
/// Single element lookup for an array value.  This is a nonterminal function.
/// The key must be an index, so that a single value lookup is returned.
/// This function can be called in path traversal.
///
/// An error is returned if the value is not an array or the key is not an index.
pub fn query_array_index(array: &Vec<Value>, idx: &IndexType) -> Result<Vec<Value>, JQError> {
    let mut results: Vec<Value> = Vec::new();
    let (indexes, _silent) = idx.as_index()?;
    for mut idx in indexes {
        // If idx is negative, pull from end of array
        if idx < 0 {
            idx += array.len() as isize;
        }
        if idx >= array.len() as isize {
            // disregard silent.  Always push null.
            // This mirrors jq behavior
            results.push(Value::Null);
            continue;
        }
        results.push(array[idx as usize].to_owned())
    }
    Ok(results)
}

fn query_string_index(input: &str, index: &IndexType) -> Result<Vec<Value>, JQError> {
    let mut results: Vec<Value> = Vec::new();
    let (indexes, silent) = index.as_index()?;
    for mut idx in indexes {
        if idx < 0 {
            idx += input.len() as isize;
        }
        if idx >= input.len() as isize {
            // idx is out of bounds
            if silent {
                continue;
            } else {
                return Err(JQError::ArrayQuery(format!("string index oob {}", idx)));
            }
        }
        results.push(Value::from(
            input.get(idx as usize..idx as usize + 1).unwrap_or(""),
        ));
    }
    Ok(results)
}
/// Query an array.  This is a terminal query operation.
///
/// Returns an error if the value is not an array, or the key is not a range.
fn query_array_range(array: &Vec<Value>, range: &RangeType) -> Result<Vec<Value>, JQError> {
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

/// Query an array.  This is a terminal query operation.
///
/// Returns an error if the value is not an array, or the key is not a range.
fn query_string_range(value: &str, range: &RangeType) -> Result<Vec<Value>, JQError> {
    if range.is_empty() {
        return Ok(vec![Value::from(value)]);
    }
    let len = value.len();
    let (start, end) = range.as_slice(len);
    let val = value.get(start..end).unwrap_or("");

    Ok(vec![Value::from(val)])
}

/// Used by [fn_has]
pub fn query_identity(input: &Value) -> Result<Vec<Value>, JQError> {
    Ok(vec![input.to_owned()])
}

/// Used by [fn_has]
pub fn query_ident(input: &Value, id: &str, silent: bool) -> Result<Vec<Value>, JQError> {
    match input {
        Value::Object(object) => query_object_ident(object, id),
        _ => {
            if silent {
                let v: Vec<Value> = Vec::new();
                return Ok(v);
            }
            Err(JQError::IdentMismatch(format!(
                "cannot index {}",
                value_name(input)
            )))
        }
    }
}

/// Used by [fn_has]
pub fn query_range(input: &Value, range: &RangeType) -> Result<Vec<Value>, JQError> {
    match input {
        Value::Object(object) => query_object_range(object, range),
        Value::Array(array) => query_array_range(array, range),
        Value::String(s) => query_string_range(s, range),
        _ => Ok(vec![input.to_owned()]),
    }
}

/// Used by [fn_has]
pub fn query_index(input: &Value, index: &IndexType) -> Result<Vec<Value>, JQError> {
    match input {
        Value::Object(object) => query_object_index(object, index),
        Value::Array(array) => query_array_index(array, index),
        Value::String(s) => query_string_index(s, index),
        _ => Ok(vec![input.to_owned()]),
    }
}

/// Query each input with the given token
fn query_single_token(inputs: &Vec<Value>, token: &Token) -> Result<Vec<Value>, JQError> {
    let mut results: Vec<Value> = Vec::new();

    for input in inputs {
        let mut result = match token {
            Token::Identity => query_identity(input)?,
            Token::Ident(ident, silent) => query_ident(input, ident, *silent)?,
            Token::Range(range) => query_range(input, range)?,
            Token::Index(index) => query_index(input, index)?,
        };
        results.append(&mut result);
    }

    Ok(results)
}

/// We need to walk the set of Filter elements, and process each.
/// Currently, we are doing this backward.  For each value, we are processing
/// the filter.  But this doesn't work if a filter element can return multiple
/// values, such as `.array[1,2]
/// For each filter element
///     execute the filter query and return the collection of results.
fn query_filter(inputs: &[Value], filter: &Filter) -> Result<Vec<Value>, JQError> {
    let mut values = inputs.to_vec();

    for token in filter {
        if token.is_identity() {
            continue;
        }
        values = query_single_token(&values, token)?;
    }
    Ok(values)
}

fn query_function(inputs: &Vec<Value>, func: Function) -> Result<Vec<Value>, JQError> {
    let mut output: Vec<Value> = Vec::new();

    let mut results = match func {
        Function::Length => fn_length(inputs)?,
        Function::Has(has) => fn_has(inputs, &has)?,
        Function::Recurse => fn_recurse(inputs)?,
        Function::Keys(sort) => fn_keys(inputs, sort)?,
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
    if block.collect {
        let collect = Value::from(results);
        results = vec![collect];
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
            collect: false,
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

        let filter = vec![
            Token::Ident("object_1", false),
            Token::Ident("elem_1", false),
        ];
        let action = Action::Filter(filter);
        let block = Block {
            actions: Some(vec![action]),
            collect: false,
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
                actions: Some(vec![Action::Filter(vec![Token::Ident("object_1", false)])]),
                collect: false,
            },
            Block {
                actions: Some(vec![Action::Filter(vec![Token::Ident("elem_1", false)])]),
                collect: false,
            },
        ];

        let result = query(&[input], blocks).expect("Failed query");

        //dbg!(&result);
        assert_eq!(&result, &[json!("Object 1 Element 1")]);
    }

    #[test]
    fn test_collect_block() {
        let json = json!([1, 2, 3]);
        let filter = vec![Token::Identity, Token::Range(RangeType::new())];
        let action = Action::Filter(filter);
        let block = Block {
            actions: Some(vec![action]),
            collect: true,
        };
        let result = query_block(&vec![json], block).expect("Failed query");

        dbg!(&result);
    }

    #[test]
    fn test_bock() {
        let json = include_str!("../test/basic.json");
        let input: Value = serde_json::from_str(json).expect("Failed to parse");

        let filter = vec![
            Token::Ident("object_1", false),
            Token::Ident("elem_1", false),
        ];
        let action = Action::Filter(filter);
        let block = Block {
            actions: Some(vec![action]),
            collect: false,
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
        let result = query_filter(&[input.clone()], &filter).expect("Failed query");

        //dbg!(&result);
        assert_eq!(&result, &vec![input]);
    }

    #[test]
    fn test_filter_object_with_identity() {
        let json = include_str!("../test/basic.json");
        let input: Value = serde_json::from_str(json).expect("Failed to parse");

        let filter = vec![
            Token::Identity,
            Token::Ident("object_1", false),
            Token::Ident("elem_1", false),
        ];

        let result = query_filter(&[input], &filter).expect("Failed query");
        assert_eq!(&result, &[json!("Object 1 Element 1")]);
    }

    #[test]
    fn test_filter_object() {
        let json = include_str!("../test/basic.json");
        let input: Value = serde_json::from_str(json).expect("Failed to parse");

        let filter = vec![
            Token::Ident("object_1", false),
            Token::Ident("elem_1", false),
        ];

        let result = query_filter(&[input], &filter).expect("Failed query");
        assert_eq!(&result, &[json!("Object 1 Element 1")]);
    }

    #[test]
    fn test_filter_array_with_identity() {
        let input = json!([{"name":"JSON", "good":true}, {"name":"XML", "good":false}]);

        let filter = vec![Token::Identity, Token::Index(IndexType::from((0, false)))];

        let result = query_filter(&[input], &filter).expect("Failed query");
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

        let filter = vec![Token::Index(IndexType::from((0, false)))];

        let result = query_filter(&[input], &filter).expect("Failed query");
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
        let value = json!({"elem1":"element 1"});
        let result =
            query_object_ident(value.as_object().unwrap(), "elem1").expect("Failed to query");
        assert_eq!(result, vec![json!("element 1")]);
    }

    #[test]
    fn test_query_nested_object_by_ident() {
        let value = json!({"object_1":{"elem1":"element 1"}});
        let result =
            query_object_ident(value.as_object().unwrap(), "object_1").expect("query failed");

        assert_eq!(result, vec![json!({"elem1":"element 1"})]);
    }

    #[test]
    fn test_object_by_index() {
        let value = json!({"object_1":{"elem1":"element 1"}});
        let index = IndexType::from(("object_1", false));
        let result = query_object_index(value.as_object().unwrap(), &index).expect("query failed");
        // dbg!(&result);
        assert_eq!(result, vec![json!({"elem1":"element 1"})]);
    }

    #[test]
    fn test_by_by_index_oob_silent() {
        let value = json!(["0", "1", "2"]);
        let index = IndexType::from((3, true));
        let result = query_array_index(value.as_array().unwrap(), &index).expect("Failed");
        //dbg!(&result);
        assert_eq!(&result, &[json!(null)]);
    }

    #[test]
    fn test_object_by_empty_index() {
        let object = json!({"a":"elem a", "b":"elem b"});
        let range = RangeType::new();
        let result = query_object_range(object.as_object().unwrap(), &range).expect("query failed");
        //dbg!(&result);
        assert_eq!(result, vec![json!("elem a"), json!("elem b")]);
    }

    #[test]
    fn test_array_by_index() {
        let value = json!(["0", "1", "2"]);
        let index = IndexType::from((0, false));
        let result = query_array_index(value.as_array().unwrap(), &index).expect("query failed");
        //dbg!(&result);
        assert_eq!(result, vec![json!("0")]);
    }

    #[test]
    fn test_array_negative_index() {
        let value = json!(["0", "1", "2"]);
        let index = IndexType::from((-2, false));
        let result = query_array_index(value.as_array().unwrap(), &index).expect("query failed");
        //dbg!(&result);
        assert_eq!(result, vec![json!("1")]);
    }

    #[test]
    fn test_array_by_index_oob() {
        let value = json!(["0", "1", "2"]);
        let index = IndexType::from((3, false));
        let result = query_array_index(value.as_array().unwrap(), &index).expect("Failed");
        //dbg!(&result);
        assert_eq!(&result, &[json!(null)]);
    }

    #[test]
    fn test_array_by_index_oob_silent() {
        let value = json!(["0", "1", "2"]);
        let index = IndexType::from((3, true));
        let result = query_array_index(value.as_array().unwrap(), &index).expect("Failed");
        //dbg!(&result);
        assert_eq!(&result, &[json!(null)]);
    }

    #[test]
    fn test_array_by_empty_range() {
        let array = json!(["0", "1", "2"]);
        let range = RangeType::new();
        let result = query_array_range(array.as_array().unwrap(), &range).expect("query failed");
        assert_eq!(result, vec![json!("0"), json!("1"), json!("2")]);
    }
}
