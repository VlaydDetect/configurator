//! Pattern validation.
//!
//! The pattern argument can be a regular expression provided as a string literal, which is then parsed by the [`regex`] crate (if the `regex` feature is enabled).
//!
//! ```rust
//! #[derive(configurator::Validate)]
//! struct Test {
//!     #[validate(pattern(r"[a-zA-Z0-9][a-zA-Z0-9_]+"))]
//!     v: String,
//! }
//! ```
//!
//! Alternatively, it can be an expression of type implementing [`Matcher`] or one that dereferences to a [`Matcher`].
//! [`Matcher`] is implemented for `regex::Regex` (if the `regex` feature is enabled) and `std::sync::LazyLock<T>` / `once_cell::sync::Lazy<T>` with any `T: Matcher`.
//! Please note that the expression will be evaluated each time `validate` is called, so avoid doing any expensive work in the expression.
//! If the work is unavoidable, at least try to amortize it, such as by using `std::sync::LazyLock` or `once_cell::Lazy`.
//!
//! ```rust
//! use std::sync::LazyLock;
//! use regex::Regex;
//!
//! static LAZY_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-zA-Z0-9][a-zA-Z0-9_]+").unwrap());
//!
//! #[derive(configurator::Validate)]
//! struct Test {
//!     #[validate(pattern(LAZY_RE))]
//!     v: String,
//! }
//! ```
//!
//! ```rust
//! use once_cell::sync::Lazy;
//! use regex::Regex;
//!
//! static LAZY_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[a-zA-Z0-9][a-zA-Z0-9_]+").unwrap());
//!
//! #[derive(configurator::Validate)]
//! struct Test {
//!     #[validate(pattern(LAZY_RE))]
//!     v: String,
//! }
//! ```
//!
//! The entrypoint is the [`Pattern`] trait. Implementing this trait for a type allows that type to be used with the `#[validate(pattern(...))]` rule.
//!
//! This trait has a blanket implementation for all `T: configurator::validator::rules::AsStr`.

use super::AsStr;
use crate::validator::error::Error;

pub fn apply<T: Pattern, M: Matcher>(v: &T, (pat,): (&M,)) -> Result<(), Error> {
    if !v.validate_pattern(pat) {
        return Err(Error::new(format!(
            "does not match pattern /{}/",
            pat.as_str()
        )));
    }
    Ok(())
}

pub trait Matcher: AsStr {
    /// Returns true if and only if there is a match for the pattern anywhere in the haystack given.
    fn is_match(&self, haystack: &str) -> bool;
}

impl<T: Matcher> Matcher for std::sync::LazyLock<T> {
    fn is_match(&self, haystack: &str) -> bool {
        std::sync::LazyLock::force(self).is_match(haystack)
    }
}

impl<T: AsStr> AsStr for std::sync::LazyLock<T> {
    fn as_str(&self) -> &str {
        std::sync::LazyLock::force(self).as_str()
    }
}

pub trait Pattern {
    fn validate_pattern<M: Matcher>(&self, matcher: &M) -> bool;
}

impl<T: AsStr> Pattern for T {
    fn validate_pattern<M: Matcher>(&self, matcher: &M) -> bool {
        matcher.is_match(self.as_str())
    }
}

impl<T: Pattern> Pattern for Option<T> {
    fn validate_pattern<M: Matcher>(&self, matcher: &M) -> bool {
        match self {
            Some(value) => value.validate_pattern(matcher),
            None => true,
        }
    }
}

#[cfg(feature = "regex")]
#[doc(hidden)]
pub mod regex {
    pub use regex::Regex;

    use super::{AsStr, Matcher};

    impl Matcher for Regex {
        fn is_match(&self, haystack: &str) -> bool {
            self.is_match(haystack)
        }
    }

    impl<T: Matcher> Matcher for once_cell::sync::Lazy<T> {
        fn is_match(&self, haystack: &str) -> bool {
            once_cell::sync::Lazy::force(self).is_match(haystack)
        }
    }

    impl AsStr for Regex {
        fn as_str(&self) -> &str {
            self.as_str()
        }
    }

    impl<T: AsStr> AsStr for once_cell::sync::Lazy<T> {
        fn as_str(&self) -> &str {
            once_cell::sync::Lazy::force(self).as_str()
        }
    }

    pub type StaticPattern = std::sync::LazyLock<Regex>;

    #[macro_export]
    macro_rules! __init_pattern {
        ($pat:literal) => {
            $crate::validator::rules::pattern::regex::StaticPattern::new(|| {
                $crate::validator::rules::pattern::regex::Regex::new($pat).unwrap()
            })
        };
    }
    pub use crate::__init_pattern as init_pattern;
}