//!
//! Array.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use super::super::LENGTH;

  impl_x2_interfaces!
  (
    [ Scalar ; LENGTH ],
    [ 0 ],
    [ 1 ],
    | _0, _1 | [ _0, _1 ],
    | _self | unsafe { std::mem::transmute::< _, _ >( _self ) }
  );
}

//

crate::mod_interface!
{
}
