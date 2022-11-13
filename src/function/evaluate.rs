// `select` function
///
//use serde_json::json;
use crate::{
    eval::*, query_ident, query_identity, query_index, query_range, ExpressionType, JQError,
    Operator, Token, Value,
};

fn evaluate_token(input: &Value, token: &Token) -> Result<Value, JQError> {
    let value = match token {
        Token::Identity => query_identity(input),
        Token::Ident(ident, silent) => query_ident(input, ident, *silent),
        Token::Range(range) => query_range(input, range),
        Token::Index(index) => query_index(input, index),
    }?;
    if value.len() != 1 {
        // TODO: Is this a legit error?
        return Err(JQError::EquationError(
            "Did not expect multiple values".to_string(),
        ));
    }
    Ok(value[0].to_owned())
}

fn eval_op(op: &Operator, left: Value, right: Value) -> Result<Value, JQError> {
    match op {
        Operator::Plus => add_value(&left, &right),
        Operator::Minus => subtract_value(&left, &right),
        Operator::Multiply => multiply_value(&left, &right),
        Operator::Divide => divide_value(&left, &right),
        _ => equality_value(op, &left, &right),
    }
}

fn eval(input: &Value, expr: &ExpressionType) -> Result<Value, JQError> {
    match expr {
        ExpressionType::Number(n) => Ok(Value::from(*n)),
        ExpressionType::String(s) => Ok(Value::from(*s)),
        ExpressionType::Ident(token) => evaluate_token(input, token),
        ExpressionType::Op(op, l, r) => eval_op(op, eval(input, l)?, eval(input, r)?),
    }
}

/// Returns boolean if the input includes the element
pub fn fn_evaluate<'a>(
    inputs: &Vec<Value>,
    expr: &ExpressionType<'a>,
) -> Result<Vec<Value>, JQError> {
    let mut results: Vec<Value> = Vec::new();

    for input in inputs {
        let result = eval(input, expr)?;
        results.push(result);
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{action::Action, jq_peg::parse};
    use serde_json::json;

    fn parse_expr(qs: &str) -> ExpressionType {
        let query = parse(qs).expect("Parse failed");
        let expr = match &query
            .get(0)
            .unwrap()
            .actions
            .as_ref()
            .unwrap()
            .get(0)
            .unwrap()
            .to_owned()
        {
            Action::Expression(expr) => expr.to_owned(),
            _ => todo!(),
        };
        expr
    }

    #[test]
    fn test_add() {
        let expr = parse_expr(r#"(.a == 2)"#);
        let inputs = vec![json!({"a": 2})];
        let results = fn_evaluate(&inputs, &expr);
        dbg!(&results);
    }
}
