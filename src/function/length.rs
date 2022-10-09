/// `length` function
///
use crate::{JQError, Value};

fn single_length(input: &Value) -> Result<Value, JQError> {
    match input {
        Value::String(s) => Ok(Value::from(s.len())),
        Value::Null => Ok(Value::from(0)),
        Value::Bool(_) => Err(JQError::FnLength("boolean has no length".to_string())),
        Value::Number(x) => Ok(Value::Number(x.to_owned())),
        Value::Array(array) => Ok(Value::from(array.len())),
        Value::Object(obj) => Ok(Value::from(obj.len())),
    }
}

/// Calculates the length of the element
///
/// For:
/// * Array: returns the number of  array elements
/// * Object: returns the number of keys
/// * String: return the length of the string
/// * Number: returns the number
/// * null: returns 0
/// * bool: returns error
pub fn fn_length(inputs: &Vec<Value>) -> Result<Vec<Value>, JQError> {
    let mut results: Vec<Value> = Vec::new();

    for input in inputs {
        let result = single_length(input)?;
        results.push(result);
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn fn_length_multi() {
        let arrays = vec![
            json!(["array_1", "array_2"]),
            json!({"a":"a", "b":"b", "c":"c"}),
            json!(null),
            json!(1),
            json!("12345"),
        ];

        let result = fn_length(&arrays).expect("fail");
        assert_eq!(
            result,
            [
                Value::from(2),
                Value::from(3),
                Value::from(0),
                Value::from(1),
                Value::from(5)
            ]
        );
    }

    #[test]
    fn fn_length_array() {
        let arrays = vec![json!(["array_1", "array_2"])];
        let result = fn_length(&arrays).expect("fail");
        assert_eq!(result, [Value::from(2)]);
    }

    #[test]
    fn fn_length_object() {
        let arrays = vec![json!({"a":"a", "b":"b", "c":"c"})];
        let result = fn_length(&arrays).expect("fail");
        assert_eq!(result, [Value::from(3)]);
    }

    #[test]
    fn fn_length_null() {
        let arrays = vec![json!(null)];
        let result = fn_length(&arrays).expect("fail");
        assert_eq!(result, [Value::from(0)]);
    }

    #[test]
    fn fn_length_bool() {
        let arrays = vec![json!(false)];
        let result = fn_length(&arrays);
        assert!(result.is_err());
    }

    #[test]
    fn fn_length_real() {
        let arrays = vec![json!(12.5)];
        let result = fn_length(&arrays).expect("fail");
        assert_eq!(result, [Value::from(12.5)]);
    }

    #[test]
    fn fn_length_integer() {
        let arrays = vec![json!(1)];
        let result = fn_length(&arrays).expect("fail");
        assert_eq!(result, [Value::from(1)]);
    }

    #[test]
    fn fn_length_string() {
        let arrays = vec![json!("12345")];
        let result = fn_length(&arrays).expect("fail");
        assert_eq!(result, [Value::from(5)]);
    }
}
