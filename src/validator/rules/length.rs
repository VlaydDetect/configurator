//! Length validation.
//!
//! ```rust
//! #[derive(configurator::Validate)]
//! struct Test {
//!     #[garde(length(min=1, max=100))]
//!     v: String,
//! }
//! ```
//!
//! The concept of "length" is somewhat complicated, especially for strings. Therefore, the `length` rule currently supports different modes:
//! - [`Simple`][simple::Simple], which is the default
//! - [`Bytes`][bytes::Bytes]
//! - [`Chars`][chars::Chars]
//! - [`Graphemes`][graphemes::Graphemes]
//! - [`Utf16CodeUnits`][utf16::Utf16CodeUnits]
//!
//! The mode is configured on the `length` rule:
//! ```rust
//! #[derive(configurator::Validate)]
//! struct Test {
//!     #[garde(
//!         length(graphemes, min=1, max=25),
//!         length(bytes, min=1, max=100),
//!     )]
//!     v: String,
//! }
//! ```
//!
//! Here's what implementing the trait for a custom string-like type might look like:
//! ```rust
//! #[repr(transparent)]
//! struct MyString(String);
//!
//! impl configurator::rules::length::HasSimpleLength for MyString {
//!     fn length(&self) -> usize {
//!         self.0.len()
//!     }
//! }
//! ```
//!
//! See each trait for more information.
//!

pub mod bytes;
pub use bytes::HasBytes;

pub mod chars;
pub use chars::HasChars;

#[cfg(feature = "unicode")]
pub mod graphemes;
#[cfg(feature = "unicode")]
pub use graphemes::HasGraphemes;

pub mod simple;
pub use simple::HasSimpleLength;

pub mod utf16;
pub use utf16::HasUtf16CodeUnits;

use crate::validator::error::Error;

fn check_len(len: usize, min: usize, max: usize) -> Result<(), Error> {
    if len < min {
        Err(Error::new(format!("length is lower than {min}")))
    } else if len > max {
        Err(Error::new(format!("length is greater than {max}")))
    } else {
        Ok(())
    }
}