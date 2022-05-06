/// Internal namespace.
pub mod internal
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

  #[
    cfg( all
    (
      not( feature = "nalgebra_ops" ),
      not( all( feature = "default_ops", feature = "nalgebra" ) ),
      any( feature = "default_ops", feature = "cgmath_ops" ),
    ))
  ]
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

  #[
    cfg( all
    (
      not( feature = "nalgebra_ops" ),
      not( all( feature = "default_ops", feature = "nalgebra" ) ),
      any( feature = "default_ops", feature = "cgmath_ops" ),
    ))
  ]
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

/// Exposed namespace of the module.
pub mod exposed
{
  use super::internal as i;
  pub use i::AsCgmathNonCanonicalInterface;
  pub use i::AsCgmathCanonicalInterface;
  pub use crate::AsForeignNonCanonicalInterface;
  pub use crate::AsForeignCanonicalInterface;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  #[ allow( unused_imports ) ]
  use super::internal as i;
  pub use i::AsCgmathNonCanonicalInterface;
  pub use i::AsCgmathCanonicalInterface;
  pub use crate::AsForeignNonCanonicalInterface;
  pub use crate::AsForeignCanonicalInterface;
}
