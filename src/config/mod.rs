pub mod value;
pub mod providers;
pub mod error;
pub mod util;
mod configurator;
mod profile;
mod coalesce;
mod metadata;
mod provider;

#[cfg(any(test, feature = "test"))] mod jail;
#[cfg(any(test, feature = "test"))] pub use jail::Jail;

#[doc(inline)]
pub use error::{Error, Result};
pub use self::configurator::Configurator;
pub use profile::Profile;
pub use provider::*;
pub use metadata::*;