/// Internal namespace.
pub mod internal
{
  // use crate::prelude::*;

  ///
  /// Trait to interpret non-canonical math data structures of other math libs as their analogs a math lib of choice to use operations of the library..
  ///

  pub trait AsNativeNonCanonicalInterface< T >
  where
    T : Copy,
    Self : Copy,
  {
    /// Clone this data structure as analog of a math lib of choice to use its operations.
    fn clone_as_native( &self ) -> T;
  }

  ///
  /// Trait to interpret canonical math data structures of other math libs as their analogs a math lib of choice to use operations of the library..
  ///

  pub trait AsNativeCanonicalInterface< T >
  where
    T : Copy,
    Self : AsNativeNonCanonicalInterface< T > + Copy,
  {
    /// Interpret this data structure as analog of a math lib of choice to use its operations.
    fn as_native( &self ) -> &T;
    /// Interpret this data structure mutably as analog of a math lib of choice to use its operations.
    fn as_native_mut( &mut self ) -> &mut T;
  }

  /* xxx : explain difference between nominal/basic/canonical */
  /* xxx : check cargo links */

}

/// Exposed namespace of the module.
pub mod exposed
{
  use super::internal as i;
  pub use i::AsNativeNonCanonicalInterface;
  pub use i::AsNativeCanonicalInterface;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  use super::internal as i;
  pub use i::AsNativeNonCanonicalInterface;
  pub use i::AsNativeCanonicalInterface;
}
