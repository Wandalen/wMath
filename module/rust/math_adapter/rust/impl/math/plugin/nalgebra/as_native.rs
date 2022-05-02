/// Internal namespace.
pub mod internal
{

  #[ allow( unused_imports ) ]
  use crate::prelude::*;

  ///
  /// Trait for non-canonical structure to clone math data structures of other math libs as their analogs in nalgebra to use operations of nalgebra.
  ///

  pub trait AsNalgebraNonCanonicalInterface< T >
  where
    T : Copy,
    Self : Copy
  {

    /// Clone this data structure as nalgebra analog to use its operations.
    fn clone_as_nalgebra( &self ) -> T;

  }

  #[ cfg( any( feature = "nalgebra_ops", feature = "default_ops" ) ) ]
  impl< T, Any > AsNativeNonCanonicalInterface< T > for Any
  where
    T : Copy,
    Any : AsNalgebraNonCanonicalInterface< T > + Copy,
  {
    fn clone_as_native( &self ) -> T
    {
      self.clone_as_nalgebra()
    }
  }

  ///
  /// Trait to interpret canonical math data structures of other math libs as their analogs in nalgebra to use operations of nalgebra.
  ///

  pub trait AsNalgebraCanonicalInterface< T >
  where
    T : Copy,
    Self : AsNalgebraNonCanonicalInterface< T > + Copy,
  {
    /// Interpret this data structure as nalgebra analog to use its operations.
    fn as_nalgebra( &self ) -> &T;
    /// Interpret this data structure mutably as nalgebra analog to use its operations.
    fn as_nalgebra_mut( &mut self ) -> &mut T;

  }

  #[ cfg( any( feature = "nalgebra_ops", feature = "default_ops" ) ) ]
  impl< T, Any > AsNativeCanonicalInterface< T > for Any
  where
    T : Copy,
    Any : AsNalgebraCanonicalInterface< T > + Copy,
  {
    /// Interpret this data structure as nalgebra analog to use its operations.
    fn as_native( &self ) -> &T
    {
      self.as_nalgebra()
    }
    /// Interpret this data structure mutably as nalgebra analog to use its operations.
    fn as_native_mut( &mut self ) -> &mut T
    {
      self.as_nalgebra_mut()
    }
  }

}

/// Exposed namespace of the module.
pub mod exposed
{
  use super::internal as i;
  pub use i::AsNalgebraNonCanonicalInterface;
  pub use i::AsNalgebraCanonicalInterface;
  pub use crate::AsNativeNonCanonicalInterface;
  pub use crate::AsNativeCanonicalInterface;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  #[ allow( unused_imports ) ]
  use super::internal as i;
  #[ cfg( any( feature = "nalgebra_ops", feature = "default_ops" ) ) ]
  pub use i::AsNalgebraNonCanonicalInterface;
  #[ cfg( any( feature = "nalgebra_ops", feature = "default_ops" ) ) ]
  pub use i::AsNalgebraCanonicalInterface;
  pub use crate::AsNativeNonCanonicalInterface;
  pub use crate::AsNativeCanonicalInterface;
}
