use crate::{JQError, Operator, Value};

fn fn_equal(left: &Value, right: &Value) -> Result<bool, JQError> {
    match left {
        Value::Null => match right {
            Value::Null => Ok(true),
            _ => Err(JQError::EquationError(
                "Cannot equate null and non-null".to_string(),
            )),
        },
        Value::Bool(b) => match right {
            Value::Bool(bb) => Ok(b == bb),
            _ => Err(JQError::EquationError(
                "Cannot equate bool and non-bool".to_string(),
            )),
        },
        Value::Number(b) => match right {
            Value::Number(bb) => Ok(b == bb),
            _ => Err(JQError::EquationError(
                "Cannot equate number and non-number".to_string(),
            )),
        },
        Value::String(b) => match right {
            Value::String(bb) => Ok(b == bb),
            _ => Err(JQError::EquationError(
                "Cannot equate String and non-String".to_string(),
            )),
        },
        Value::Array(b) => match right {
            Value::Array(bb) => Ok(b == bb),
            _ => Err(JQError::EquationError(
                "Cannot equate Array and non-Array".to_string(),
            )),
        },
        Value::Object(b) => match right {
            Value::Object(bb) => Ok(b == bb),
            _ => Err(JQError::EquationError(
                "Cannot equate Object and non-Object".to_string(),
            )),
        },
    }
}

fn fn_not_equal(left: &Value, right: &Value) -> Result<bool, JQError> {
    match left {
        Value::Null => Err(JQError::EquationError("Cannot not-equate null".to_string())),
        Value::Bool(b) => match right {
            Value::Bool(bb) => Ok(b != bb),
            _ => Err(JQError::EquationError(
                "Cannot equate bool and non-bool".to_string(),
            )),
        },
        Value::Number(b) => match right {
            Value::Number(bb) => Ok(b != bb),
            _ => Err(JQError::EquationError(
                "Cannot equate number and non-number".to_string(),
            )),
        },
        Value::String(b) => match right {
            Value::String(bb) => Ok(b != bb),
            _ => Err(JQError::EquationError(
                "Cannot equate String and non-String".to_string(),
            )),
        },
        Value::Array(b) => match right {
            Value::Array(bb) => Ok(b != bb),
            _ => Err(JQError::EquationError(
                "Cannot equate Array and non-Array".to_string(),
            )),
        },
        Value::Object(b) => match right {
            Value::Object(bb) => Ok(b != bb),
            _ => Err(JQError::EquationError(
                "Cannot equate Object and non-Object".to_string(),
            )),
        },
    }
}

fn fn_gt(left: &Value, right: &Value) -> Result<bool, JQError> {
    match left {
        Value::Null => Err(JQError::EquationError(
            "Null cannot be greater than Null".to_string(),
        )),
        Value::Bool(_) => Err(JQError::EquationError(
            "Boolean cannot be greater than Boolean".to_string(),
        )),
        Value::Number(b) => match right {
            Value::Number(bb) => Ok(b.as_f64().unwrap() > bb.as_f64().unwrap()),
            _ => Err(JQError::EquationError(
                "Cannot equate number and non-number".to_string(),
            )),
        },
        Value::String(b) => match right {
            Value::String(bb) => Ok(b > bb),
            _ => Err(JQError::EquationError(
                "Cannot equate String and non-String".to_string(),
            )),
        },
        Value::Array(b) => match right {
            Value::Array(bb) => Ok(b.len() > bb.len()),
            _ => Err(JQError::EquationError(
                "Cannot equate Array and non-Array".to_string(),
            )),
        },
        Value::Object(_) => Err(JQError::EquationError(
            "Object cannot be greater than Object".to_string(),
        )),
    }
}

fn fn_lt(left: &Value, right: &Value) -> Result<bool, JQError> {
    match left {
        Value::Null => Err(JQError::EquationError(
            "Null cannot be less than Null".to_string(),
        )),
        Value::Bool(_) => Err(JQError::EquationError(
            "Boolean cannot be greater than Boolean".to_string(),
        )),
        Value::Number(b) => match right {
            Value::Number(bb) => Ok(b.as_f64().unwrap() < bb.as_f64().unwrap()),
            _ => Err(JQError::EquationError(
                "Cannot equate number and non-number".to_string(),
            )),
        },
        Value::String(b) => match right {
            Value::String(bb) => Ok(b < bb),
            _ => Err(JQError::EquationError(
                "Cannot equate String and non-String".to_string(),
            )),
        },
        Value::Array(b) => match right {
            Value::Array(bb) => Ok(b.len() < bb.len()),
            _ => Err(JQError::EquationError(
                "Cannot equate Array and non-Array".to_string(),
            )),
        },
        Value::Object(_) => Err(JQError::EquationError(
            "Object cannot be greater than Object".to_string(),
        )),
    }
}

fn fn_gte(left: &Value, right: &Value) -> Result<bool, JQError> {
    match left {
        Value::Null => Err(JQError::EquationError(
            "Null cannot be greater than or equal to Null".to_string(),
        )),
        Value::Bool(_) => Err(JQError::EquationError(
            "Boolean cannot be greater than or equal to Boolean".to_string(),
        )),
        Value::Number(b) => match right {
            Value::Number(bb) => Ok(b.as_f64().unwrap() >= bb.as_f64().unwrap()),
            _ => Err(JQError::EquationError(
                "Cannot equate number and non-number".to_string(),
            )),
        },
        Value::String(b) => match right {
            Value::String(bb) => Ok(b >= bb),
            _ => Err(JQError::EquationError(
                "Cannot equate String and non-String".to_string(),
            )),
        },
        Value::Array(b) => match right {
            Value::Array(bb) => Ok(b.len() >= bb.len()),
            _ => Err(JQError::EquationError(
                "Cannot equate Array and non-Array".to_string(),
            )),
        },
        Value::Object(_) => Err(JQError::EquationError(
            "Object cannot be less than or equal to Object".to_string(),
        )),
    }
}

fn fn_lte(left: &Value, right: &Value) -> Result<bool, JQError> {
    match left {
        Value::Null => Err(JQError::EquationError(
            "Null cannot be less than or equal to Null".to_string(),
        )),
        Value::Bool(_) => Err(JQError::EquationError(
            "Boolean cannot be less than or equal to Boolean".to_string(),
        )),
        Value::Number(b) => match right {
            Value::Number(bb) => Ok(b.as_f64().unwrap() <= bb.as_f64().unwrap()),
            _ => Err(JQError::EquationError(
                "Cannot equate number and non-number".to_string(),
            )),
        },
        Value::String(b) => match right {
            Value::String(bb) => Ok(b <= bb),
            _ => Err(JQError::EquationError(
                "Cannot equate String and non-String".to_string(),
            )),
        },
        Value::Array(b) => match right {
            Value::Array(bb) => Ok(b.len() <= bb.len()),
            _ => Err(JQError::EquationError(
                "Cannot equate Array and non-Array".to_string(),
            )),
        },
        Value::Object(_) => Err(JQError::EquationError(
            "Object cannot be less than or equal to Object".to_string(),
        )),
    }
}

///
pub fn equality_value(op: &Operator, left: &Value, right: &Value) -> Result<Value, JQError> {
    match op {
        Operator::Equal => Ok(Value::from(fn_equal(left, right)?)),
        Operator::NotEqual => Ok(Value::from(fn_not_equal(left, right)?)),
        Operator::Gt => Ok(Value::from(fn_gt(left, right)?)),
        Operator::Lt => Ok(Value::from(fn_lt(left, right)?)),
        Operator::Gte => Ok(Value::from(fn_gte(left, right)?)),
        Operator::Lte => Ok(Value::from(fn_lte(left, right)?)),
        _ => Err(JQError::EquationError(
            "Wrong operator for equality".to_string(),
        )),
    }
}
