/// Internal namespace.
pub mod internal
{
  use core::ops::{ Neg, Add, Sub };
  use crate::prelude::*;
  use crate::X2;
  use crate::impl_x2_rented_op1;
  use crate::impl_x2_rented_op2;

  impl_x2_rented_op1!( Neg, neg, cgmath::Vector2 );
  impl_x2_rented_op2!( Add, add, cgmath::Vector2 );
  impl_x2_rented_op2!( Sub, sub, cgmath::Vector2 );

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
