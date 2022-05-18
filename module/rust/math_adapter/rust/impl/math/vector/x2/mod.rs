/// Internal namespace.
mod internal
{
  // use core::fmt::Debug;
  use core::hash::Hash;
  use crate::ScalarInterface;

  // #[ cfg( any( feature = "cgmath_ops", feature = "nalgebra_ops" ) ) ]

  include!( "./impl_deref.rs" );
  include!( "./impl_rented_op.rs" );

  include!( "./array.rs" );
  include!( "./interface.rs" );
  include!( "./slice.rs" );
  include!( "./struct.rs" );
  include!( "./tuple.rs" );

}

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod macros
{
  use super::internal as i;
  pub( crate ) use i::impl_x2_deref;
  pub( crate ) use i::impl_x2_rented_op1;
  pub( crate ) use i::impl_x2_rented_op2;
}
/* qqq : implement alll operators */

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod protected
{
  pub use super::exposed::*;
  use super::internal as i;
  pub use super::macros::*;
}

pub use protected::*;

/// Exposed namespace of the module.
pub mod exposed
{
  use super::internal as i;
  pub use i::X2;
  pub use i::X2NominalInterface as X2Interface;
  pub use i::X2NominalInterface;
  pub use i::X2BasicInterface;
  pub use i::X2CanonicalInterface;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  use super::internal as i;
  pub use i::X2;
  pub use i::X2NominalInterface as X2Interface;
  pub use i::X2NominalInterface;
  pub use i::X2BasicInterface;
  pub use i::X2CanonicalInterface;
}
