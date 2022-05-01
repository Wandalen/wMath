/// Internal namespace.
pub mod internal
{
  // use core::mem::size_of;
  // use crate::prelude::*;
  // use crate::X2;
  // use crate::ScalarInterface;

  // include!( "./x2.rs" );
  // impl_x2_for!( cgmath::Vector2< Scalar > );

  /// X2 Vector of cgmath
  pub type X2< Scalar > = cgmath::Vector2< Scalar >;

}

/// Trait to interpret math data structures of other math libs as their analogs in cgmath to use operations of cgmath.
pub mod as_cgmath;
#[ cfg( feature = "cgmath_ops" ) ]
/// Use cgmath's operations.
pub mod ops;
/// Implement interfaces for objects of the math library.
pub mod x2;

/// Own namespace of the module.
pub mod own
{
  use super::internal as i;
  pub use i::X2;
}

pub use own::*;

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::as_cgmath::exposed::*;
  #[ cfg( feature = "cgmath_ops" ) ]
  pub use super::ops::exposed::*;
  pub use super::x2::exposed::*;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use super::as_cgmath::prelude::*;
  #[ cfg( feature = "cgmath_ops" ) ]
  pub use super::ops::prelude::*;
  pub use super::x2::prelude::*;
}
