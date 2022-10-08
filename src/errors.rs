use thiserror::Error;

#[derive(Error, Debug)]
pub enum JQError {
    #[error("Error: {0}")]
    GeneralError(String),
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("JSON parsing error ")]
    JSONError(#[from] serde_json::Error),
    #[error("Parse error")]
    ParseError,
    #[error("Action Mismatch, expecting {0}")]
    ActionMismatch(String),
    #[error("Token Mismatch, expecting {0}")]
    TokenMismatch(String),
    #[error("This Value is not an object")]
    NotAnObject,
    #[error("This Value is not an array")]
    NotAnArray,
    #[error("Value variant is not supported for this function")]
    UnsupportedValue,
    #[error("Element does not support range operations")]
    UnsupportedRange,
    #[error("Range boundary error: {0}:{1}-{2}")]
    RangeOutOfBounds(isize, isize, isize),
    #[error("Element does not support string index")]
    UnsupportedObjectIndex,
    #[error("Unexpected IndexType")]
    BadIndexType,
    #[error("Keytype must have either identifier or range")]
    MalformedIndexType,
    #[error("Error querying object: {0}")]
    ObjectQuery(String),
    #[error("Error querying array: {0}")]
    ArrayQuery(String),
}
