/// Internal namespace.
pub mod internal
{
  use core::fmt::Debug;
  use core::hash::Hash;
  use crate::ScalarInterface;

  include!( "./x2/array.rs" );
  include!( "./x2/interface.rs" );
  include!( "./x2/ops.rs" );
  // #[ cfg( any( feature = "cgmath_ops", feature = "nalgebra_ops" ) ) ]
  include!( "./x2/rented_op.rs" );
  include!( "./x2/struct.rs" );
  include!( "./x2/tuple.rs" );

}

/// Exposed namespace of the module.
pub mod exposed
{
  use super::internal as i;
  pub use i::X2;
  pub use i::X2Interface;
  pub use i::X2CanonicalInterface;
  #[ cfg( any( feature = "cgmath_ops", feature = "nalgebra_ops" ) ) ]
  pub( crate ) use i::impl_x2_rented_op1;
  #[ cfg( any( feature = "cgmath_ops", feature = "nalgebra_ops" ) ) ]
  pub( crate ) use i::impl_x2_rented_op2;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  use super::internal as i;
  pub use i::X2Interface;
  pub use i::X2CanonicalInterface;
}
