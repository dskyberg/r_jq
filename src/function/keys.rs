/// `keys` and `keys_unsorted` functions
///
use crate::{JQError, Value};

/// keys return a sorted set of key values
pub fn fn_keys(values: &Vec<Value>, sort: bool) -> Result<Vec<Value>, JQError> {
    let mut results: Vec<Value> = Vec::new();

    for value in values {
        if let Some(object) = value.as_object() {
            if sort {
                let mut keys: Vec<String> =
                    object.keys().into_iter().map(|s| s.to_owned()).collect();
                keys.sort();
                let keys = Value::from(
                    keys.iter()
                        .map(|s| Value::from(s.to_owned()))
                        .collect::<Vec<Value>>(),
                );
                results.push(keys);
            } else {
                let keys = Value::from(
                    object
                        .keys()
                        .into_iter()
                        .map(|s| Value::from(s.to_owned()))
                        .collect::<Vec<Value>>(),
                );
                results.push(keys);
            }
        } else if let Some(array) = value.as_array() {
            let mut result: Vec<Value> = Vec::new();
            for idx in 0..array.len() {
                result.push(Value::from(idx));
            }
            results.push(Value::from(result));
        } else {
            return Err(JQError::GeneralError(
                "keys can only be used for objects".to_string(),
            ));
        }
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_keys() {
        let value = json!({"abc": 1, "abcd": 2, "Foo": 3});

        let result = fn_keys(&vec![value], true).expect("failed");
        //dbg!(&result);
        assert_eq!(result, &[json!(["Foo", "abc", "abcd"])]);
    }

    #[test]
    fn test_keys_unsorted() {
        let value = json!({"abc": 1, "abcd": 2, "Foo": 3});

        let result = fn_keys(&vec![value], false).expect("failed");
        //dbg!(&result);
        assert_eq!(result, &[json!(["abc", "abcd", "Foo"])]);
    }
    #[test]
    fn test_keys_with_array() {
        let value = json!([42, 3, 5]);

        let result = fn_keys(&vec![value], true).expect("failed");
        //dbg!(&result);
        assert_eq!(result, &[json!([0, 1, 2])]);
    }
}
