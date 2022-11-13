use crate::{JQError, Value};

///
pub fn divide_number(left: &Value, right: &Value) -> Result<Value, JQError> {
    match left.as_f64() {
        Some(ln) => match right {
            Value::Null => Ok(Value::from(ln)),
            Value::Bool(_) => Err(JQError::EquationError(
                "number and boolean cannot be divided".to_string(),
            )),
            Value::Number(_) => match right.as_f64() {
                Some(rn) => Ok(Value::from(ln / rn)),
                _ => Err(JQError::EquationError("right is not a number".to_string())),
            },
            Value::String(_) => Err(JQError::EquationError(
                "number and string cannot be divided".to_string(),
            )),
            Value::Array(_) => Err(JQError::EquationError(
                "number and array cannot be divided".to_string(),
            )),
            Value::Object(_) => Err(JQError::EquationError(
                "number and object cannot be divided".to_string(),
            )),
        },
        _ => Err(JQError::EquationError("Left is not a number".to_string())),
    }
}

///
pub fn divide_value(left: &Value, right: &Value) -> Result<Value, JQError> {
    match left {
        Value::Number(_) => divide_number(left, right),
        Value::String(_) => Err(JQError::EquationError(
            "String cannot be divided".to_string(),
        )),
        Value::Array(_) => Err(JQError::EquationError(
            "Array cannot be divided".to_string(),
        )),
        Value::Object(_) => Err(JQError::EquationError(
            "Object cannot be divided".to_string(),
        )),
        Value::Bool(_) => Err(JQError::EquationError(
            "Boolean cannot be divided".to_string(),
        )),
        Value::Null => Ok(right.clone()),
    }
}
