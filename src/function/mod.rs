use crate::HasType;
pub use has::*;
pub use length::*;
pub use recurse::*;

/// `has`
pub mod has;
/// `length`
pub mod length;

/// `recurse`
pub mod recurse;

/// Represents a Function in the PEG parser
#[derive(Clone, Debug, PartialEq)]
pub enum Function<'a> {
    /// [fn_length]
    Length,
    /// [fn_has]
    Has(HasType<'a>),
    /// Recursive descent
    Recurse,
}
