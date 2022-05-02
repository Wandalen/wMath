/// Internal namespace.
pub mod internal
{

  ///
  /// Trait for non-canonical structure to clone math data structures of other math libs as their analogs in nalgebra to use operations of nalgebra.
  ///

  pub trait AsNalgebraNonCanonicalInterface< T >
  {

    /// Clone this data structure as nalgebra analog to use its operations.
    fn clone_as_nalgebra( &self ) -> T;

    /// Clone this data structure as nalgebra analog to use its operations.
    fn clone_as_native( &self ) -> T
    {
      self.clone_as_nalgebra()
    }

  }

  ///
  /// Trait to interpret canonical math data structures of other math libs as their analogs in nalgebra to use operations of nalgebra.
  ///

  pub trait AsNalgebraCanonicalInterface< T >
  {
    /// Interpret this data structure as nalgebra analog to use its operations.
    fn as_nalgebra( &self ) -> &T;
    /// Interpret this data structure mutably as nalgebra analog to use its operations.
    fn as_nalgebra_mut( &mut self ) -> &mut T;

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
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  use super::internal as i;
  pub use i::AsNalgebraNonCanonicalInterface;
  pub use i::AsNalgebraCanonicalInterface;
}
