pub mod error;
mod rc_list;
pub mod rules;
pub mod validate;

#[cfg(feature = "derive")]
pub use configurator_derive::{Validate, select};
pub use error::{Error, Path, Report};
pub use validate::{Unvalidated, Valid, Validate};

pub type Result = ::core::result::Result<(), Error>;

pub mod external {
	pub use {compact_str, smallvec};
}

#[doc(hidden)]
pub mod util {
	use super::Path;
	use super::error::PathComponentKind;

	#[inline]
	pub fn __make_nested_path<'a, C: PathComponentKind + Clone + 'a>(
		mut parent: impl FnMut() -> Path + 'a,
		component: C,
	) -> impl FnMut() -> Path + 'a {
		let mut nested = None::<Path>;

		move || MaybeJoin::maybe_join(&mut nested, &mut parent, || component.clone())
	}

	#[doc(hidden)]
	#[macro_export]
	macro_rules! __nested_path {
		($parent:ident, $key:expr) => {
			$crate::validator::util::__make_nested_path(&mut $parent, &$key)
		};
	}

	pub use crate::__nested_path as nested_path;

	pub trait MaybeJoin {
		fn maybe_join<C, P, CF>(&mut self, parent: P, component: CF) -> Path
		where
			C: PathComponentKind,
			P: FnMut() -> Path,
			CF: Fn() -> C;
	}

	impl MaybeJoin for Option<Path> {
		#[inline]
		fn maybe_join<C, P, CF>(&mut self, mut parent: P, component: CF) -> Path
		where
			C: PathComponentKind,
			P: FnMut() -> Path,
			CF: Fn() -> C,
		{
			match self {
				Some(path) => path.clone(),
				None => {
					let path = parent().join(component());
					*self = Some(path.clone());
					path
				}
			}
		}
	}
}
