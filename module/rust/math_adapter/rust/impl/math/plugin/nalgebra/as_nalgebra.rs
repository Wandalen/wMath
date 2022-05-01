/// Internal namespace.
pub mod internal
{
  // use core::fmt::Debug;
  // use core::hash::Hash;
  // use core::ops::Mul;
  // use crate::ScalarInterface;
  // use crate::X2CanonicalInterface;
  // use crate::prelude::*;

  ///
  /// Trait to interpret math data structures of other math libs as their analogs in nalgebra to use operations of nalgebra.
  ///

  pub trait AsNalgebraInterface< T >
  {
    /// Interpret this data structure as nalgebra analog to use its operations.
    fn as_nalgebra( &self ) -> &T;
    /// Interpret this data structure mutably as nalgebra analog to use its operations.
    fn as_nalgebra_mut( &mut self ) -> &mut T;
    // /// Clone this data structure as nalgebra analog to use its operations.
    // fn clone_as_nalgebra( &self ) -> T;
  }

  //

  // impl< Scalar, AsNalgebra, Right > Mul< Right >
  // for AsNalgebra
  // where
  //   Scalar : ScalarInterface,
  //   AsNalgebra : AsNalgebraInterface< Scalar > + X2CanonicalInterface< Scalar = Scalar >,
  //   Right : AsNalgebraInterface< Scalar > + X2CanonicalInterface< Scalar = Scalar >,
  // {
  //   type Output = Self;
  //   fn mul( self, right : Right ) -> Self::Output
  //   {
  //     self.as_nalgebra() * right.as_nalgebra()
  //   }
  // }

  //

  // impl< Right, Scalar > Mul< &Right >
  // for &AsNalgebra< Scalar >
  // where
  //   AsNalgebra : AsNalgebraInterface< Scalar >,
  //   Scalar : ScalarInterface,
  //   Right : X2Interface< Scalar = Scalar > + Copy,
  // {
  //   type Output = < AsNalgebra< Scalar > as Mul< Right > >::Output;
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
  pub use i::AsNalgebraInterface;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
  // pub use i::X2Interface;
}
