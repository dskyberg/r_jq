/// `has` function
///
use crate::{query_array_element, query_object_element, HasType, JQError, Token, Value};

/// Returns boolean if the input includes the element
pub fn fn_has<'a>(inputs: &Vec<Value>, has: &HasType<'a>) -> Result<Vec<Value>, JQError> {
    let mut results: Vec<Value> = Vec::new();
    let h = has.clone();
    let token = Token::from(&h);

    for input in inputs {
        let result = match input {
            Value::Object(_) => query_object_element(input, &token),
            Value::Array(_) => query_array_element(input, &token),
            _ => Err(JQError::UnsupportedValue),
        };
        if result.is_ok() && !result.unwrap().is_null() {
            results.push(Value::Bool(true));
        } else {
            results.push(Value::Bool(false))
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HasType;
    use serde_json::json;

    #[test]
    fn test_object_has() {
        let objects = vec![json!({"elem1":"element 1"}), json!({"elem_1":"element 1"})];
        let key = HasType::from("elem1");
        let result = fn_has(&objects, &key).expect("Failed to query");
        // dbg!(result);
        assert_eq!(result, vec![json!(true), json!(false)]);
    }

    #[test]
    fn test_array_has() {
        let arrays = vec![json!(["array_1", "array_2"]), json!(["array_1"])];
        let key = HasType::from(1);
        let result = fn_has(&arrays, &key).expect("Failed to query");
        //dbg!(&result);
        assert_eq!(result, vec![json!(true), json!(false)]);
    }
}
