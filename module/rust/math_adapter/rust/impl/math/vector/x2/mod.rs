/// Internal namespace.
pub( crate ) mod private
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
  use super::private as i;
  pub( crate ) use i::impl_x2_deref;
  pub( crate ) use i::impl_x2_rented_op1;
  pub( crate ) use i::impl_x2_rented_op2;
}
/* qqq : implement alll operators */

// crate::mod_interface!
// {
//   protected use super::macros::*;
//
//   prelude use X2;
//   prelude use X2NominalInterface as X2Interface;
//   prelude use X2NominalInterface;
//   prelude use X2BasicInterface;
//   prelude use X2CanonicalInterface;
//
// }

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod protected
{
  pub use super::exposed::*;
  // use super::internal as i;
  pub use super::macros::*;
}

pub use protected::*;

/// Exposed namespace of the module.
pub mod exposed
{
  // use super::internal as i;
  pub use super::private::X2;
  pub use super::private::X2NominalInterface as X2Interface;
  pub use super::private::X2NominalInterface;
  pub use super::private::X2BasicInterface;
  pub use super::private::X2CanonicalInterface;
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
  pub use super::private::X2;
  pub use super::private::X2NominalInterface as X2Interface;
  pub use super::private::X2NominalInterface;
  pub use super::private::X2BasicInterface;
  pub use super::private::X2CanonicalInterface;
}
