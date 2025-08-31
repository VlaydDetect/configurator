//! Field matching validation.
//!
//! ```rust
//! #[derive(configurator::Validate)]
//! struct Test {
//!     #[validate(skip)]
//!     foo: String,
//!     #[validate(matches(foo))]
//!     bar: String,
//! }
//! ```
//!
//! The entrypoint is the [`Matches`] trait. Implementing this trait for a type allows that type to be used with the `#[validate(matches)]` rule.
//!
//! This trait has a blanket implementation for all `T: PartialEq<O>, O`.

use crate::validator::Error;

pub fn apply<T: Matches<O>, O>(v: &T, (field, value): (&str, &O)) -> Result<(), Error> {
    if !v.validate_matches(value) {
        return Err(Error::new(format!("does not match {field} field")));
    }
    Ok(())
}

pub trait Matches<O> {
    fn validate_matches(&self, other: &O) -> bool;
}

impl<T: PartialEq<O>, O> Matches<O> for T {
    fn validate_matches(&self, other: &O) -> bool {
        self.eq(other)
    }
}