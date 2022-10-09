use crate::HasType;
pub use has::*;
pub use length::*;

/// `has`
pub mod has;
/// `length`
pub mod length;

/// Represents a Function in the PEG parser
#[derive(Clone, Debug, PartialEq)]
pub enum Function<'a> {
    /// [fn_length]
    Length,
    /// [fn_has]
    Has(HasType<'a>),
}
