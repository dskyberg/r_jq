// `select` function
///
use serde_json::json;

use crate::{
    eval::*, query_ident, query_identity, query_index, query_range, AtomType, ExpressionType,
    JQError, Operator, Token, Value,
};

fn get_value_from_token(input: &Value, token: &Token) -> Result<Value, JQError> {
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

fn reduce(input: &Value, atom: &AtomType) -> Result<Value, JQError> {
    let result = match atom {
        AtomType::String(s) => Value::String(s.to_string()),
        AtomType::Number(n) => json!(n),
        AtomType::Token(token) => get_value_from_token(input, token)?,
        AtomType::Expression(expr) => evaluate(input, expr)?,
    };
    Ok(result)
}

///
pub fn evaluate(input: &Value, expr: &ExpressionType) -> Result<Value, JQError> {
    // Recursively resolve left and right expressions
    let mut left = reduce(input, &expr.left)?;
    let right = reduce(input, &expr.right)?;

    match expr.operator {
        Operator::Plus => add_value(&left, &right),
        Operator::Minus => subtract_value(&left, &right),
        Operator::Multiply => todo!(),
        Operator::Divide => todo!(),
        Operator::Equal => equate_value(&left, &right, false),
        Operator::NotEqual => equate_value(&left, &right, true),
        Operator::Gt => todo!(),
        Operator::Lt => todo!(),
        Operator::Gte => todo!(),
        Operator::Lte => todo!(),
    }
}

/// Returns boolean if the input includes the element
pub fn fn_evaluate<'a>(
    inputs: &Vec<Value>,
    _expression: &ExpressionType<'a>,
) -> Result<Vec<Value>, JQError> {
    let mut results: Vec<Value> = Vec::new();

    for _input in inputs {}
    Ok(results)
}
