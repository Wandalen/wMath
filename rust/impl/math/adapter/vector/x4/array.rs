//!
//! Array.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use super::super::LENGTH;

  impl_interfaces!
  (
    [ Scalar ; LENGTH ],
    [ 0 ],
    [ 1 ],
    [ 2 ],
    [ 3 ],
    | _0, _1, _2, _3 | [ _0, _1, _2, _3 ],
    | _self | unsafe { std::mem::transmute::< _, _ >( _self ) }
  );
}

//

crate::mod_interface!
{
}
