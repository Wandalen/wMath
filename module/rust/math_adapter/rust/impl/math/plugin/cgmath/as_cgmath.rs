/// Internal namespace.
pub mod internal
{
  // use core::fmt::Debug;
  // use core::hash::Hash;
  // use core::ops::Mul;
  // use crate::ScalarInterface;
  // use crate::x2_canonical_interface;
  // use crate::prelude::*;

  ///
  /// Trait to interpret math data structures of other math libs as their analogs in cgmath to use operations of cgmath.
  ///

  pub trait AsCgmathInterface< T >
  {
    /// Interpret this data structure as cgmath analog to use its operations.
    fn as_cgmath( &self ) -> &T;
    /// Interpret this data structure mutably as cgmath analog to use its operations.
    fn as_cgmath_mut( &mut self ) -> &mut T;
    // /// Clone this data structure as cgmath analog to use its operations.
    // fn clone_as_cgmath( &self ) -> T;
    // xxx
  }

  //

  // impl< Scalar, AsCgmath, Right > Mul< Right >
  // for AsCgmath
  // where
  //   Scalar : ScalarInterface,
  //   AsCgmath : AsCgmathInterface< Scalar > + x2_canonical_interface< Scalar = Scalar >,
  //   Right : AsCgmathInterface< Scalar > + x2_canonical_interface< Scalar = Scalar >,
  // {
  //   type Output = Self;
  //   fn mul( self, right : Right ) -> Self::Output
  //   {
  //     self.as_cgmath() * right.as_cgmath()
  //   }
  // }

  //

  // impl< Right, Scalar > Mul< &Right >
  // for &AsCgmath< Scalar >
  // where
  //   AsCgmath : AsCgmathInterface< Scalar >,
  //   Scalar : ScalarInterface,
  //   Right : x2_interface< Scalar = Scalar > + Copy,
  // {
  //   type Output = < AsCgmath< Scalar > as Mul< Right > >::Output;
  //   fn add( self, right : &Right ) -> Self::Output
  //   {
  //     Add::< Right >::mul( *self, *right )
  //   }
  // }

}

/// Exposed namespace of the module.
pub mod exposed
{
  use super::internal as i;
  pub use i::AsCgmathInterface;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
  // pub use i::x2_interface;
}
