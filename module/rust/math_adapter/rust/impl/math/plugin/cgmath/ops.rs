/// Internal namespace.
pub( crate ) mod private
{
  use core::ops::{ Neg, Add, Sub };
  use crate::prelude::*;
  use crate::X2;
  use crate::vector::{ impl_x2_rented_op1, impl_x2_rented_op2, impl_vector_deref };
  use core::ops::{ Deref, DerefMut };

  impl_x2_rented_op1!( Neg, neg, cgmath::Vector2 );
  impl_x2_rented_op2!( Add, add, cgmath::Vector2 );
  impl_x2_rented_op2!( Sub, sub, cgmath::Vector2 );
  /* qqq : implement more operators. don't forget about tests */

  impl_vector_deref!( cgmath::Vector2, X2 );

}

crate::mod_interface!
{
}
