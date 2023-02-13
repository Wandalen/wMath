//!
//! Vector of length 4.
//!

/// Internal namespace.
pub( crate ) mod private
{
}

const LENGTH : usize = 4;

crate::mod_interface!
{

  layer array;
  layer slice;
  layer tuple;
  layer structure;

}
