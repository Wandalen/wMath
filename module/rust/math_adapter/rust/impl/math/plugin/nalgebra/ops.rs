/// Internal namespace.
mod internal
{
  use core::ops::{ Neg, Add, Sub };
  use crate::prelude::*;
  use crate::X2;
  use crate::vector::{ impl_x2_rented_op1, impl_x2_rented_op2, impl_x2_deref };
  use core::ops::{ Deref, DerefMut };

  impl_x2_rented_op1!( Neg, neg, nalgebra::Vector2 );
  impl_x2_rented_op2!( Add, add, nalgebra::Vector2 );
  impl_x2_rented_op2!( Sub, sub, nalgebra::Vector2 );
  /* qqq : implement more operators. don't forget about tests */

  impl_x2_deref!( nalgebra::Vector2 );

  // impl< Scalar, Src > Deref for Src
  // where
  //   Src : X2Interface< Scalar = Scalar >,
  //   Scalar : ScalarInterface,
  // {
  //   type Target = nalgebra::Vector2< Scalar >;
  //   fn deref( &self ) -> &Self::Target
  //   {
  //     self.as_foreign()
  //   }
  // }

  // impl< Scalar > DerefMut for X2< Scalar >
  // where
  //   Scalar : ScalarInterface,
  // {
  //   fn deref_mut( &mut self ) -> &mut Self::Target
  //   {
  //     self.as_foreign_mut()
  //   }
  // }

}

/// Exposed namespace of the module.
pub mod exposed
{
  // use super::internal as i;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
}
