use serde_json::Value;

//use crate::Block;
use crate::{query_array_element, query_object_element, Function, JQError, Token};

pub fn has(inputs: &Vec<Value>, command: &Function) -> Result<Vec<Value>, JQError> {
    let mut results: Vec<Value> = Vec::new();
    let token = Token::try_from(command.to_owned())?;

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
    use crate::Function;
    use serde_json::json;

    #[test]
    fn test_object_has() {
        let objects = vec![json!({"elem1":"element 1"}), json!({"elem_1":"element 1"})];
        let index = None;
        let ident = Some("elem1");
        let key = Function::Has { index, ident };
        let result = has(&objects, &key).expect("Failed to query");
        // dbg!(result);
        assert_eq!(result, vec![json!(true), json!(false)]);
    }

    #[test]
    fn test_array_has() {
        let arrays = vec![json!(["array_1", "array_2"]), json!(["array_1"])];
        let index = Some(1);
        let ident = None;
        let key = Function::Has { index, ident };
        let result = has(&arrays, &key).expect("Failed to query");
        dbg!(result);
        //assert_eq!(result, vec![json!(true), json!(false)]);
    }
}
