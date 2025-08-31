//! Substring validation.
//!
//! ```rust
//! const STR: &str = "test";
//!
//! #[derive(configurator::Validate)]
//! struct Test {
//!     #[validate(contains("test"))]
//!     v: String,
//!     #[validate(contains(STR))]
//!     w: String,
//! }
//! ```
//!
//! The entrypoint is the [`Contains`] trait. Implementing this trait for a type allows that type to be used with the `#[validate(contains)]` rule.
//!
//! This trait has a blanket implementation for all `T: configurator::validator::rules::AsStr`.

use super::AsStr;
use crate::validator::error::Error;

pub fn apply<T: Contains>(v: &T, (pat,): (&str,)) -> Result<(), Error> {
    if !v.validate_contains(pat) {
        return Err(Error::new(format!("does not contain \"{pat}\"")));
    }
    Ok(())
}

pub trait Contains {
    fn validate_contains(&self, pat: &str) -> bool;
}

impl<T: AsStr> Contains for T {
    fn validate_contains(&self, pat: &str) -> bool {
        self.as_str().contains(pat)
    }
}

impl<T: Contains> Contains for Option<T> {
    fn validate_contains(&self, pat: &str) -> bool {
        match self {
            Some(value) => value.validate_contains(pat),
            None => true,
        }
    }
}