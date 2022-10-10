use crate::{JQError, Value};

fn recurse_single_value(value: &Value) -> Result<Vec<Value>, JQError> {
    let mut results: Vec<Value> = vec![value.to_owned()];

    if let Some(object) = value.as_object() {
        for (_key, val) in object {
            let mut vals = recurse_single_value(val)?;
            results.append(&mut vals);
        }
    } else if let Some(array) = value.as_array() {
        for val in array {
            let mut vals = recurse_single_value(val)?;
            results.append(&mut vals);
        }
    }

    Ok(results)
}

/// Recursively descend objects. This is only intened to be used on
/// Identity, `.`
pub fn fn_recurse(values: &Vec<Value>) -> Result<Vec<Value>, JQError> {
    let mut results: Vec<Value> = Vec::new();

    for value in values {
        let mut result = recurse_single_value(value)?;
        results.append(&mut result);
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_fn_recurse() {
        let values = vec![json!([[{"a":1}]])];
        let result = fn_recurse(&values).expect("Failed");
        //dbg!(&result);
        assert_eq!(
            result,
            &[
                json!([
                  [
                    {
                      "a": 1
                    }
                  ]
                ]),
                json!([
                  {
                    "a": 1
                  }
                ]),
                json!({
                  "a": 1
                }),
                json!(1)
            ]
        );
    }
}
