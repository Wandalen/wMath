/// Internal namespace.
pub( crate ) mod private
{
  use core::ops::{ Neg, Add, Sub };
  use crate::prelude::*;
  use crate::
  {
    X2,
    X3,
    X4,
  };
  use crate::vector::{ impl_rented_op1, impl_rented_op2, impl_vector_deref };
  use core::ops::{ Deref, DerefMut };

  impl_rented_op1!( Neg, neg, cgmath::Vector2, X2 );
  impl_rented_op2!( Add, add, cgmath::Vector2, X2 );
  impl_rented_op2!( Sub, sub, cgmath::Vector2, X2 );

  impl_rented_op1!( Neg, neg, cgmath::Vector3, X3 );
  impl_rented_op2!( Add, add, cgmath::Vector3, X3 );
  impl_rented_op2!( Sub, sub, cgmath::Vector3, X3 );

  impl_rented_op1!( Neg, neg, cgmath::Vector4, X4 );
  impl_rented_op2!( Add, add, cgmath::Vector4, X4 );
  impl_rented_op2!( Sub, sub, cgmath::Vector4, X4 );
  /* qqq : implement more operators. don't forget about tests */

  impl_vector_deref!( cgmath::Vector2, X2 );

  impl_vector_deref!( cgmath::Vector3, X3 );

  impl_vector_deref!( cgmath::Vector4, X4 );
}

crate::mod_interface!
{
}
