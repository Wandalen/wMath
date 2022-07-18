/// Internal namespace.
pub( crate ) mod private
{
  // use crate::*;


}

/* qqq : implement all operators */

crate::mod_interface!
{

  /// Macro to implement deref trait.
  layer impl_deref;
  /// Macro to implement rented operators.
  layer impl_rented_op;

  /// Array.
  layer array;
  /// Slice.
  layer slice;
  /// Struct.
  layer structure;
  /// Tuple.
  layer tuple;

}
