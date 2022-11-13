use crate::{JQError, Operator, Value};

fn fn_equal(left: &Value, right: &Value) -> Result<bool, JQError> {
    match left {
        Value::Null => match right {
            Value::Null => Ok(true),
            _ => Ok(false),
        },
        Value::Bool(b) => match right {
            Value::Bool(bb) => Ok(b == bb),
            _ => Ok(false),
        },
        Value::Number(b) => match right {
            Value::Number(bb) => Ok(b == bb),
            _ => Ok(false),
        },
        Value::String(b) => match right {
            Value::String(bb) => Ok(b == bb),
            _ => Ok(false),
        },
        Value::Array(b) => match right {
            Value::Array(bb) => Ok(b == bb),
            _ => Ok(false),
        },
        Value::Object(b) => match right {
            Value::Object(bb) => Ok(b == bb),
            _ => Ok(false),
        },
    }
}

fn fn_not_equal(left: &Value, right: &Value) -> Result<bool, JQError> {
    match left {
        Value::Null => match right {
            Value::Null => Ok(false),
            _ => Ok(true),
        },
        Value::Bool(b) => match right {
            Value::Bool(bb) => Ok(b != bb),
            _ => Ok(true),
        },
        Value::Number(b) => match right {
            Value::Number(bb) => Ok(b != bb),
            _ => Ok(true),
        },
        Value::String(b) => match right {
            Value::String(bb) => Ok(b != bb),
            _ => Ok(true),
        },
        Value::Array(b) => match right {
            Value::Array(bb) => Ok(b != bb),
            _ => Ok(true),
        },
        Value::Object(b) => match right {
            Value::Object(bb) => Ok(b != bb),
            _ => Ok(true),
        },
    }
}

fn fn_gt(left: &Value, right: &Value) -> Result<bool, JQError> {
    match left {
        Value::Null => Ok(false),
        Value::Bool(b) => match right {
            Value::Bool(bb) => Ok(*b & !(*bb)),
            _ => Ok(false),
        },
        Value::Number(n) => match right {
            Value::Number(nn) => Ok(n.as_f64().unwrap() > nn.as_f64().unwrap()),
            Value::Bool(_) => Ok(true),
            _ => Ok(false),
        },
        Value::String(b) => match right {
            Value::String(bb) => Ok(b > bb),
            _ => Ok(false),
        },
        Value::Array(b) => match right {
            Value::Array(bb) => Ok(b.len() > bb.len()),
            _ => Ok(false),
        },
        Value::Object(_) => Ok(false),
    }
}

fn fn_lt(left: &Value, right: &Value) -> Result<bool, JQError> {
    match left {
        Value::Null => match right {
            Value::Null => Ok(false),
            _ => Ok(true),
        },
        Value::Bool(b) => match right {
            Value::Bool(bb) => Ok(!(*b) & *bb),
            _ => Ok(false),
        },
        Value::Number(n) => match right {
            Value::Number(nn) => Ok(n.as_f64().unwrap() < nn.as_f64().unwrap()),
            _ => Ok(false),
        },
        Value::String(b) => match right {
            Value::String(bb) => Ok(b < bb),
            _ => Ok(false),
        },
        Value::Array(b) => match right {
            Value::Array(bb) => Ok(b.len() < bb.len()),
            _ => Ok(false),
        },
        Value::Object(_) => Ok(false),
    }
}

fn fn_gte(left: &Value, right: &Value) -> Result<bool, JQError> {
    match left {
        Value::Null => Ok(true),
        Value::Bool(b) => match right {
            Value::Bool(bb) => Ok(*b >= *bb),
            _ => Ok(false),
        },
        Value::Number(n) => match right {
            Value::Number(nn) => Ok(n.as_f64().unwrap() >= nn.as_f64().unwrap()),
            Value::Bool(_) => Ok(true),
            _ => Ok(false),
        },
        Value::String(b) => match right {
            Value::String(bb) => Ok(b >= bb),
            _ => Ok(false),
        },
        Value::Array(b) => match right {
            Value::Array(bb) => Ok(b.len() >= bb.len()),
            _ => Ok(false),
        },
        Value::Object(_) => Ok(false),
    }
}

fn fn_lte(left: &Value, right: &Value) -> Result<bool, JQError> {
    match left {
        Value::Null => Ok(true),
        Value::Bool(b) => match right {
            Value::Bool(bb) => Ok(*b <= *bb),
            _ => Ok(true),
        },
        Value::Number(n) => match right {
            Value::Number(nn) => Ok(n.as_f64().unwrap() <= nn.as_f64().unwrap()),
            _ => Ok(true),
        },
        Value::String(b) => match right {
            Value::String(bb) => Ok(b <= bb),
            _ => Ok(true),
        },
        Value::Array(b) => match right {
            Value::Array(bb) => Ok(b.len() <= bb.len()),
            _ => Ok(true),
        },
        Value::Object(_) => Ok(true),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Operator;
    use serde_json::json;

    #[test]
    fn test_equal() {
        assert_eq!(
            equality_value(&Operator::Equal, &json!(null), &json!(null)).expect("failed"),
            json!(true)
        );

        assert_eq!(
            equality_value(&Operator::Equal, &json!(true), &json!(true)).expect("failed"),
            json!(true)
        );

        assert_eq!(
            equality_value(&Operator::Equal, &json!(1), &json!(1)).expect("failed"),
            json!(true)
        );

        assert_eq!(
            equality_value(&Operator::Equal, &json!(r#"abc""#), &json!(r#"abc""#)).expect("failed"),
            json!(true)
        );

        assert_eq!(
            equality_value(&Operator::Equal, &json!([1, 2, 3]), &json!([1, 2, 3])).expect("failed"),
            json!(true)
        );

        assert_eq!(
            equality_value(
                &Operator::Equal,
                &json!({"a":1, "b":2, "c":3}),
                &json!({"a":1, "b":2, "c":3})
            )
            .expect("failed"),
            json!(true)
        );
    }

    #[test]
    fn test_not_equal() {
        assert_eq!(
            equality_value(&Operator::NotEqual, &json!(null), &json!(null)).expect("failed"),
            json!(false)
        );
        assert_eq!(
            equality_value(&Operator::NotEqual, &json!(null), &json!(true)).expect("failed"),
            json!(true)
        );

        assert_eq!(
            equality_value(&Operator::NotEqual, &json!(true), &json!(false)).expect("failed"),
            json!(true)
        );

        assert_eq!(
            equality_value(&Operator::NotEqual, &json!(1), &json!(0)).expect("failed"),
            json!(true)
        );

        assert_eq!(
            equality_value(&Operator::NotEqual, &json!(r#"abc""#), &json!(r#"ab""#))
                .expect("failed"),
            json!(true)
        );

        assert_eq!(
            equality_value(&Operator::NotEqual, &json!([1, 2, 3]), &json!([1, 2])).expect("failed"),
            json!(true)
        );

        assert_eq!(
            equality_value(
                &Operator::NotEqual,
                &json!({"a":1, "b":2, "c":3}),
                &json!({"a":1, "b":2})
            )
            .expect("failed"),
            json!(true)
        );
    }
}
