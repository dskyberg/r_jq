use crate::{JQError, Value};

///
pub fn equate_value(left: &Value, right: &Value, not: bool) -> Result<Value, JQError> {
    match not {
        true => Ok(Value::from(left != right)),
        false => Ok(Value::from(left == right)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strings_equal() {
        let left = Value::from("abc");
        let right = Value::from("abc");
        let result = equate_value(&left, &right, false).expect("failed");
        assert_eq!(&result, &Value::from(true));
    }

    #[test]
    fn test_strings_not_equal() {
        let left = Value::from("abc");
        let right = Value::from("def");
        let result = equate_value(&left, &right, true).expect("failed");
        assert_eq!(&result, &Value::from(true));
    }

    #[test]
    fn test_different_types_equal() {
        let left = Value::from("abc");
        let right = Value::from(4.0);
        let result = equate_value(&left, &right, false).expect("failed");
        assert_eq!(&result, &Value::from(false));
    }

    #[test]
    fn test_different_types_not_equal() {
        let left = Value::from("abc");
        let right = Value::from(4.0);
        let result = equate_value(&left, &right, true).expect("failed");
        assert_eq!(&result, &Value::from(true));
    }
}
