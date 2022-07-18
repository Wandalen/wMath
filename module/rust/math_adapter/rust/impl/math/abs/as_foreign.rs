//!
//! Interfaces to either conver or reinterpret nath data structure as analog of math lib of choice..
//!

/// Internal namespace.
pub( crate ) mod private
{
  // use crate::prelude::*;

  ///
  /// Trait to interpret non-canonical math data structures of other math libs as their analogs a math lib of choice to use operations of the library..
  ///

  pub trait AsForeignNonCanonicalInterface< T >
  where
    T : Copy,
    Self : Copy,
  {
    /// Clone this data structure as analog of a math lib of choice to use its operations.
    fn clone_as_foreign( &self ) -> T;
  }

  ///
  /// Trait to interpret canonical math data structures of other math libs as their analogs a math lib of choice to use operations of the library..
  ///

  pub trait AsForeignCanonicalInterface< T >
  where
    T : Copy,
    Self : AsForeignNonCanonicalInterface< T > + Copy,
  {
    /// Interpret this data structure as analog of a math lib of choice to use its operations.
    fn as_foreign( &self ) -> &T;
    /// Interpret this data structure mutably as analog of a math lib of choice to use its operations.
    fn as_foreign_mut( &mut self ) -> &mut T;
  }

}

crate::mod_interface!
{
  prelude use AsForeignNonCanonicalInterface;
  prelude use AsForeignCanonicalInterface;
}
