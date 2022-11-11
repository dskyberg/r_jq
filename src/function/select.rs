// `select` function
///
use crate::{AtomType, JQError, Value};

/// Returns boolean if the input includes the element
pub fn fn_select<'a>(_inputs: &[Value], _select: &AtomType<'a>) -> Result<Vec<Value>, JQError> {
    let results: Vec<Value> = Vec::new();

    Ok(results)
}
