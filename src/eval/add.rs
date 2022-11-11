use crate::{JQError, Value};

///
pub fn add_bool(left: &Value, right: &Value) -> Result<Value, JQError> {
    match left.as_bool() {
        Some(lb) => match right {
            Value::Null => Ok(Value::from(lb)),
            Value::Bool(_) => Err(JQError::EquationError(
                "boolean and boolean cannot be added".to_string(),
            )),
            Value::Number(_) => Err(JQError::EquationError(
                "boolean and number cannot be added".to_string(),
            )),
            Value::String(_) => Err(JQError::EquationError(
                "boolean and string cannot be added".to_string(),
            )),
            Value::Array(_) => Err(JQError::EquationError(
                "boolean and array cannot be added".to_string(),
            )),
            Value::Object(_) => Err(JQError::EquationError(
                "boolean and object cannot be added".to_string(),
            )),
        },
        _ => Err(JQError::EquationError("Left is not a bool".to_string())),
    }
}

///
pub fn add_number(left: &Value, right: &Value) -> Result<Value, JQError> {
    match left.as_f64() {
        Some(ln) => match right {
            Value::Null => Ok(Value::from(ln)),
            Value::Bool(_) => Err(JQError::EquationError(
                "number and boolean cannot be added".to_string(),
            )),
            Value::Number(_) => match right.as_f64() {
                Some(rn) => Ok(Value::from(ln + rn)),
                _ => Err(JQError::EquationError("right is not a number".to_string())),
            },
            Value::String(_) => Err(JQError::EquationError(
                "number and string cannot be added".to_string(),
            )),
            Value::Array(_) => Err(JQError::EquationError(
                "number and array cannot be added".to_string(),
            )),
            Value::Object(_) => Err(JQError::EquationError(
                "number and object cannot be added".to_string(),
            )),
        },
        _ => Err(JQError::EquationError("Left is not a number".to_string())),
    }
}

///
pub fn add_string(left: &Value, right: &Value) -> Result<Value, JQError> {
    match left.as_f64() {
        Some(ls) => match right {
            Value::Null => Ok(Value::from(ls)),
            Value::Bool(_) => Err(JQError::EquationError(
                "string and boolean cannot be added".to_string(),
            )),
            Value::Number(_) => Err(JQError::EquationError(
                "string and number cannot be added".to_string(),
            )),
            Value::String(rs) => Ok(Value::from(format!("{}{}", ls, rs))),
            Value::Array(_) => Err(JQError::EquationError(
                "string and array cannot be added".to_string(),
            )),
            Value::Object(_) => Err(JQError::EquationError(
                "string and object cannot be added".to_string(),
            )),
        },
        _ => Err(JQError::EquationError("Left is not a string".to_string())),
    }
}

///
pub fn add_null(left: &Value, right: &Value) -> Result<Value, JQError> {
    match left.as_null() {
        Some(_) => Ok(right.clone()),
        _ => Err(JQError::EquationError("Left is not a null".to_string())),
    }
}

///
pub fn add_array(left: &Value, right: &Value) -> Result<Value, JQError> {
    match left.as_array() {
        Some(la) => match right {
            Value::Null => Ok(left.clone()),
            Value::Bool(_) => Err(JQError::EquationError(
                "array and boolean cannot be added".to_string(),
            )),
            Value::Number(_) => Err(JQError::EquationError(
                "array and number cannot be added".to_string(),
            )),
            Value::String(_) => Err(JQError::EquationError(
                "array and string cannot be added".to_string(),
            )),
            Value::Object(_) => Err(JQError::EquationError(
                "array and object cannot be added".to_string(),
            )),
            Value::Array(ra) => {
                let mut newl = la.clone();
                newl.append(&mut ra.to_owned());
                Ok(Value::from(newl))
            }
        },
        _ => Err(JQError::EquationError("Left is not an array".to_string())),
    }
}

///
pub fn add_object(left: &Value, right: &Value) -> Result<Value, JQError> {
    match left.as_object() {
        Some(lo) => match right {
            Value::Null => Ok(left.clone()),
            Value::Bool(_) => Err(JQError::EquationError(
                "object and boolean cannot be added".to_string(),
            )),
            Value::Number(_) => Err(JQError::EquationError(
                "object and number cannot be added".to_string(),
            )),
            Value::String(_) => Err(JQError::EquationError(
                "object and string cannot be added".to_string(),
            )),
            Value::Array(_) => Err(JQError::EquationError(
                "object and array cannot be added".to_string(),
            )),
            Value::Object(ro) => {
                let mut new = lo.to_owned();
                new.extend(ro.to_owned());
                Ok(Value::from(new))
            }
        },
        _ => Err(JQError::EquationError("Left is not an array".to_string())),
    }
}

///
pub fn add_value(left: &Value, right: &Value) -> Result<Value, JQError> {
    match left {
        Value::Number(_) => add_number(left, right),
        Value::String(_) => add_string(left, right),
        Value::Array(_) => add_string(left, right),
        Value::Object(_) => add_object(left, right),
        Value::Bool(_) => add_bool(left, right),
        Value::Null => add_null(left, right),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_array() {
        let left = Value::from(vec![1, 2, 3]);
        let right = Value::from(vec![4, 5, 6]);
        let result = add_value(&left, &right).expect("failed");
        dbg!(&result);
    }
}
