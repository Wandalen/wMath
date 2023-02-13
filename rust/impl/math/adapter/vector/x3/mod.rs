//!
//! Vector of length 3.
//!

/// Internal namespace.
pub( crate ) mod private
{
}

const LENGTH : usize = 3;

crate::mod_interface!
{

  layer array;
  layer slice;
  layer tuple;
  layer structure;

}
