/// Internal namespace.
pub( crate ) mod private
{

  #[ allow( unused_imports ) ]
  use crate::prelude::*;

  ///
  /// Trait for non-canonical structure to clone math data structures of other math libs as their analogs in cgmath to use operations of cgmath.
  ///

  pub trait AsCgmathNonCanonicalInterface< T >
  where
    T : Copy,
    Self : Copy
  {

    /// Clone this data structure as cgmath analog to use its operations.
    fn clone_as_cgmath( &self ) -> T;

  }

  #[ cfg( cgmath_ops ) ]
  impl< T, Any > AsForeignNonCanonicalInterface< T > for Any
  where
    T : Copy,
    Any : AsCgmathNonCanonicalInterface< T > + Copy,
  {
    fn clone_as_foreign( &self ) -> T
    {
      self.clone_as_cgmath()
    }
  }

  ///
  /// Trait to interpret canonical math data structures of other math libs as their analogs in cgmath to use operations of cgmath.
  ///

  pub trait AsCgmathCanonicalInterface< T >
  where
    T : Copy,
    Self : AsCgmathNonCanonicalInterface< T > + Copy,
  {
    /// Interpret this data structure as cgmath analog to use its operations.
    fn as_cgmath( &self ) -> &T;
    /// Interpret this data structure mutably as cgmath analog to use its operations.
    fn as_cgmath_mut( &mut self ) -> &mut T;

  }

  #[ cfg( cgmath_ops ) ]
  impl< T, Any > AsForeignCanonicalInterface< T > for Any
  where
    T : Copy,
    Any : AsCgmathCanonicalInterface< T > + Copy,
  {
    /// Interpret this data structure as cgmath analog to use its operations.
    fn as_foreign( &self ) -> &T
    {
      self.as_cgmath()
    }
    /// Interpret this data structure mutably as cgmath analog to use its operations.
    fn as_foreign_mut( &mut self ) -> &mut T
    {
      self.as_cgmath_mut()
    }
  }

}

crate::mod_interface!
{

  // exposed use
  // {
  //   AsCgmathNonCanonicalInterface,
  //   AsCgmathCanonicalInterface,
  // };

  // #[ cfg( cgmath_ops ) ]
  prelude use
  {
    AsCgmathNonCanonicalInterface,
    AsCgmathCanonicalInterface,
  };

  // prelude use AsCgmathNonCanonicalInterface;
  // prelude use AsCgmathCanonicalInterface;
  // prelude use crate::AsForeignNonCanonicalInterface;
  // prelude use crate::AsForeignCanonicalInterface;

}
