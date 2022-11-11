use crate::JQError;

/// Represents an operator in an expression
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operator {
    ///
    Plus,
    ///
    Minus,
    ///
    Multiply,
    ///
    Divide,
    ///
    Equal,
    ///
    NotEqual,
    ///
    Gt,
    ///
    Lt,
    ///
    Gte,
    ///
    Lte,
}


impl TryFrom<&str> for Operator {
    type Error = JQError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Operator::Plus),
            "-" => Ok(Operator::Minus),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            "==" => Ok(Operator::Equal),
            "!=" => Ok(Operator::NotEqual),
            ">" => Ok(Operator::Gt),
            "<" => Ok(Operator::Lt),
            ">=" => Ok(Operator::Gte),
            "<=" => Ok(Operator::Lte),
            _ => Err(JQError::EquationError("Bad operator".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let plus = Operator::try_from("+").expect("Failed!");
        assert_eq!(plus, Operator::Plus);
    }
}
