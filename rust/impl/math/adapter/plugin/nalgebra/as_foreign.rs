//!
//! Trait to interpret math data structures of other math libs as their analogs in nalgebra to use operations of nalgebra.
//!

/// Internal namespace.
pub( crate ) mod private
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

  #[ cfg( nalgebra_ops ) ]
  impl< T, Any > AsForeignNonCanonicalInterface< T > for Any
  where
    T : Copy,
    Any : AsNalgebraNonCanonicalInterface< T > + Copy,
  {
    fn clone_as_foreign( &self ) -> T
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

  #[ cfg( nalgebra_ops ) ]
  impl< T, Any > AsForeignCanonicalInterface< T > for Any
  where
    T : Copy,
    Any : AsNalgebraCanonicalInterface< T > + Copy,
  {
    /// Interpret this data structure as nalgebra analog to use its operations.
    fn as_foreign( &self ) -> &T
    {
      self.as_nalgebra()
    }
    /// Interpret this data structure mutably as nalgebra analog to use its operations.
    fn as_foreign_mut( &mut self ) -> &mut T
    {
      self.as_nalgebra_mut()
    }
  }

}

crate::mod_interface!
{

  // exposed use
  // {
  //   AsNalgebraNonCanonicalInterface,
  //   AsNalgebraCanonicalInterface,
  // };

  // #[ cfg( nalgebra_ops ) ]
  prelude use
  {
    AsNalgebraNonCanonicalInterface,
    AsNalgebraCanonicalInterface,
  };

  // prelude use crate::
  // {
  //   AsForeignNonCanonicalInterface,
  //   AsForeignCanonicalInterface,
  // };

}
