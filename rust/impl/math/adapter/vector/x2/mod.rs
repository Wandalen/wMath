//!
//! Vector of length 2.
//!

/// Internal namespace.
pub( crate ) mod private
{
  // use crate::*;


}

const LENGTH : usize = 2;

crate::mod_interface!
{

  layer impl_deref;
  layer impl_rented_op;
  layer impl_x2_interfaces;

  layer array;
  layer slice;
  layer tuple;
  layer structure;

}
