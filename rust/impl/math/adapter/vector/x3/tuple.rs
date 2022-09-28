//!
//! Implement interfaces for tuple.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  impl_interfaces!
  (
    ( Scalar, Scalar, Scalar ),
    .0,
    .1,
    .2,
    | _0, _1, _2 | ( _0, _1, _2 ),
    | _self | unsafe { std::mem::transmute::< _, _ >( _self ) }
  );
}

//

crate::mod_interface!
{
}
