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

  layer array;
  layer slice;
  layer tuple;
  layer structure;

}
