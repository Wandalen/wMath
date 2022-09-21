//!
//! Implement interfaces for tuple.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  impl_x2_interfaces!
  (
    ( Scalar, Scalar ),
    .0,
    .1,
    | _0, _1 | ( _0, _1 ),
    | _self | unsafe { std::mem::transmute::< _, _ >( _self ) }
  );

}

//

crate::mod_interface!
{
}
