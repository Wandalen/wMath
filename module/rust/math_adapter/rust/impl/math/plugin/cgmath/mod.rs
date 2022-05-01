/// Internal namespace.
pub mod internal
{
  use core::mem::size_of;
  use crate::prelude::*;
  use crate::X2;
  use crate::ScalarInterface;

  include!( "./x2.rs" );
  impl_x2_for!( cgmath::Vector2< Scalar > );

}

/// Trait to interpret math data structures of other math libs as their analogs in cgmath to use operations of cgmath.
pub mod as_cgmath;
#[cfg( feature = "cgmath_ops" )]
/// Use cgmath's operations.
pub mod ops;

/// Exposed namespace of the module.
pub mod exposed
{
  // use super::internal as i;
  pub use super::as_cgmath::exposed::*;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
  pub use super::as_cgmath::prelude::*;
}
