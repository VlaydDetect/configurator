//! Inner type validation.
//!
//! ```rust
//! #[derive(configurator::Validate)]
//! struct Test {
//!     #[validate(inner(length(min=1)))]
//!     v: Vec<String>,
//! }
//! ```
//!
//! The entrypoint is the [`Inner`] trait. Implementing this trait for a type allows that type to be used with the `#[validate(inner(..))]` rule.

use crate::validator::error::{NoKey, PathComponentKind};

pub fn apply<T, U, K, F>(field: &T, f: F)
where
    T: Inner<U, Key = K>,
    F: FnMut(&U, &K),
{
    field.validate_inner(f)
}

pub trait Inner<T> {
    type Key: PathComponentKind;

    fn validate_inner<F>(&self, f: F)
    where
        F: FnMut(&T, &Self::Key);
}

impl<T> Inner<T> for Vec<T> {
    type Key = usize;

    fn validate_inner<F>(&self, f: F)
    where
        F: FnMut(&T, &Self::Key),
    {
        self.as_slice().validate_inner(f)
    }
}

impl<const N: usize, T> Inner<T> for [T; N] {
    type Key = usize;

    fn validate_inner<F>(&self, f: F)
    where
        F: FnMut(&T, &Self::Key),
    {
        self.as_slice().validate_inner(f)
    }
}

impl<T> Inner<T> for &[T] {
    type Key = usize;

    fn validate_inner<F>(&self, mut f: F)
    where
        F: FnMut(&T, &Self::Key),
    {
        for (index, item) in self.iter().enumerate() {
            f(item, &index);
        }
    }
}

impl<T> Inner<T> for Option<T> {
    type Key = NoKey;

    fn validate_inner<F>(&self, mut f: F)
    where
        F: FnMut(&T, &Self::Key),
    {
        if let Some(item) = self {
            f(item, &NoKey::default())
        }
    }
}