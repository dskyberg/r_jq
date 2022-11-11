use crate::{JQError, Value};

///
pub fn subtract_bool(left: &Value, right: &Value) -> Result<Value, JQError> {
    match left.as_bool() {
        Some(lb) => match right {
            Value::Null => Ok(Value::from(lb)),
            Value::Bool(_) => Err(JQError::EquationError(
                "boolean and boolean cannot be subtracted".to_string(),
            )),
            Value::Number(_) => Err(JQError::EquationError(
                "boolean and number cannot be subtracted".to_string(),
            )),
            Value::String(_) => Err(JQError::EquationError(
                "boolean and string cannot be subtracted".to_string(),
            )),
            Value::Array(_) => Err(JQError::EquationError(
                "boolean and array cannot be subtracted".to_string(),
            )),
            Value::Object(_) => Err(JQError::EquationError(
                "boolean and object cannot be subtracted".to_string(),
            )),
        },
        _ => Err(JQError::EquationError("Left is not a boolean".to_string())),
    }
}

///
pub fn subtract_number(left: &Value, right: &Value) -> Result<Value, JQError> {
    match left.as_f64() {
        Some(ln) => match right {
            Value::Null => Ok(Value::from(ln)),
            Value::Bool(_) => Err(JQError::EquationError(
                "number and boolean cannot be subtracted".to_string(),
            )),
            Value::Number(_) => match right.as_f64() {
                Some(rn) => Ok(Value::from(ln - rn)),
                _ => Err(JQError::EquationError("right is not a number".to_string())),
            },
            Value::String(_) => Err(JQError::EquationError(
                "number and string cannot be subtracted".to_string(),
            )),
            Value::Array(_) => Err(JQError::EquationError(
                "number and array cannot be subtracted".to_string(),
            )),
            Value::Object(_) => Err(JQError::EquationError(
                "number and object cannot be subtracted".to_string(),
            )),
        },
        _ => Err(JQError::EquationError("Left is not a number".to_string())),
    }
}

///
pub fn subtract_string(left: &Value, right: &Value) -> Result<Value, JQError> {
    match left.as_f64() {
        Some(ls) => match right {
            Value::Null => Ok(Value::from(ls)),
            Value::Bool(_) => Err(JQError::EquationError(
                "string and boolean cannot be subtracted".to_string(),
            )),
            Value::Number(_) => Err(JQError::EquationError(
                "string and number cannot be subtracted".to_string(),
            )),
            Value::String(_) => Err(JQError::EquationError(
                "string and string cannot be subtracted".to_string(),
            )),
            Value::Array(_) => Err(JQError::EquationError(
                "string and array cannot be subtracted".to_string(),
            )),
            Value::Object(_) => Err(JQError::EquationError(
                "string and object cannot be subtracted".to_string(),
            )),
        },
        _ => Err(JQError::EquationError("Left is not a string".to_string())),
    }
}

///
pub fn subtract_null(left: &Value, right: &Value) -> Result<Value, JQError> {
    match left.as_null() {
        Some(_) => match right {
            Value::Null => Err(JQError::EquationError(
                "null and null cannot be subtracted".to_string(),
            )),
            Value::Bool(_) => Err(JQError::EquationError(
                "null and boolean cannot be subtracted".to_string(),
            )),
            Value::Number(_) => Err(JQError::EquationError(
                "null and number cannot be subtracted".to_string(),
            )),
            Value::String(_) => Err(JQError::EquationError(
                "null and string cannot be subtracted".to_string(),
            )),
            Value::Array(_) => Err(JQError::EquationError(
                "null and array cannot be subtracted".to_string(),
            )),
            Value::Object(_) => Err(JQError::EquationError(
                "null and object cannot be subtracted".to_string(),
            )),
        },
        _ => Err(JQError::EquationError("Left is not a null".to_string())),
    }
}

///
pub fn subtract_array(left: &Value, right: &Value) -> Result<Value, JQError> {
    match left.as_array() {
        Some(la) => match right {
            Value::Null => Ok(left.clone()),
            Value::Bool(_) => Err(JQError::EquationError(
                "array and boolean cannot be subtracted".to_string(),
            )),
            Value::Number(_) => Err(JQError::EquationError(
                "array and number cannot be subtracted".to_string(),
            )),
            Value::String(_) => Err(JQError::EquationError(
                "array and string cannot be subtracted".to_string(),
            )),
            Value::Object(_) => Err(JQError::EquationError(
                "array and object cannot be subtracted".to_string(),
            )),
            Value::Array(ra) => {
                let difference: Vec<Value> = la
                    .iter()
                    .filter(|item| !ra.contains(item))
                    .map(|v| v.to_owned())
                    .collect();
                Ok(Value::from(difference))
            }
        },
        _ => Err(JQError::EquationError("Left is not an array".to_string())),
    }
}

///
pub fn subtract_object(left: &Value, right: &Value) -> Result<Value, JQError> {
    match left.as_object() {
        Some(_) => match right {
            Value::Null => Ok(left.clone()),
            Value::Bool(_) => Err(JQError::EquationError(
                "object and boolean cannot be subtracted".to_string(),
            )),
            Value::Number(_) => Err(JQError::EquationError(
                "object and number cannot be subtracted".to_string(),
            )),
            Value::String(_) => Err(JQError::EquationError(
                "object and string cannot be subtracted".to_string(),
            )),
            Value::Array(_) => Err(JQError::EquationError(
                "object and array cannot be subtracted".to_string(),
            )),
            Value::Object(_) => Err(JQError::EquationError(
                "object and object cannot be subtracted".to_string(),
            )),
        },
        _ => Err(JQError::EquationError("Left is not an array".to_string())),
    }
}

///
pub fn subtract_value(left: &Value, right: &Value) -> Result<Value, JQError> {
    match left {
        Value::Number(_) => subtract_number(left, right),
        Value::String(_) => subtract_string(left, right),
        Value::Array(_) => subtract_string(left, right),
        Value::Object(_) => subtract_object(left, right),
        Value::Null => subtract_null(left, right),
        Value::Bool(_) => subtract_bool(left, right),
    }
}
