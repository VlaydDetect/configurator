//! URL validation using the [`url`] crate.
//!
//! ```rust
//! #[derive(configurator::Validate)]
//! struct Test {
//!     #[validate(url)]
//!     v: String,
//! }
//! ```
//!
//! The entrypoint is the [`Url`] trait. Implementing this trait for a type allows that type to be used with the `#[validate(url)]` rule.
//!
//! The [`url`] crate only allows parsing from a `&str`, which is why this trait has a blanket implementation for all `T: configurator::validator::rules::AsStr`.
//!
//! If you need to implement this for a string-like type where a contiguous slice of the entire contents cannot be obtained,
//! then there is currently no way for you to implement this trait.

use std::fmt::Display;

use super::AsStr;
use crate::validator::error::Error;

pub fn apply<T: Url>(v: &T, _: ()) -> Result<(), Error> {
    if let Err(e) = v.validate_url() {
        return Err(Error::new(format!("not a valid url: {e}")));
    }
    Ok(())
}

pub trait Url {
    type Error: Display;

    fn validate_url(&self) -> Result<(), Self::Error>;
}

impl<T: AsStr> Url for T {
    type Error = url::ParseError;

    fn validate_url(&self) -> Result<(), Self::Error> {
        let _ = url::Url::parse(self.as_str())?;
        Ok(())
    }
}

impl<T: Url> Url for Option<T> {
    type Error = T::Error;

    fn validate_url(&self) -> Result<(), Self::Error> {
        match self {
            Some(value) => value.validate_url(),
            None => Ok(()),
        }
    }
}