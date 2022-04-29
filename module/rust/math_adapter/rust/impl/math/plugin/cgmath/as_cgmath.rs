/// Internal namespace.
pub mod internal
{

  ///
  /// Trait for non-canonical structure to clone math data structures of other math libs as their analogs in cgmath to use operations of cgmath.
  ///

  pub trait AsCgmathNonCanonicalInterface< T >
  {
    /// Clone this data structure as cgmath analog to use its operations.
    fn clone_as_cgmath( &self ) -> T;
  }

  ///
  /// Trait to interpret canonical math data structures of other math libs as their analogs in cgmath to use operations of cgmath.
  ///

  pub trait AsCgmathCanonicalInterface< T >
  {
    /// Interpret this data structure as cgmath analog to use its operations.
    fn as_cgmath( &self ) -> &T;
    /// Interpret this data structure mutably as cgmath analog to use its operations.
    fn as_cgmath_mut( &mut self ) -> &mut T;
  }

}

/// Exposed namespace of the module.
pub mod exposed
{
  use super::internal as i;
  pub use i::AsCgmathNonCanonicalInterface;
  pub use i::AsCgmathCanonicalInterface;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
  // pub use i::x2_interface;
}
