use thiserror::Error;

#[derive(Error, Debug)]
pub enum JQError {
    #[error("Parse error")]
    ParseError,
    #[error("Token Mismatch, expecting {0}")]
    TokenMismatch(String),
    #[error("This Value is not an object")]
    NotAnObject,
    #[error("This Value is not an array")]
    NotAnArray,
    #[error("Element does not support range operations")]
    UnsupportedRange,
    #[error("Range boundary error: {0}:{1}-{2}")]
    RangeOutOfBounds(isize, isize, isize),
    #[error("Element does not support string index")]
    UnsupportedObjectIndex,
    #[error("Keytype cannot have both identifier and range")]
    BadKeyType,
    #[error("Keytype must have either identifier or range")]
    MalformedKeyType,
}
