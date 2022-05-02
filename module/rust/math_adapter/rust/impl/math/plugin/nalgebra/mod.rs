/// Internal namespace.
pub mod internal
{
  /// X2 Vector of nalgebra
  pub type X2< Scalar > = nalgebra::Vector2< Scalar >;

}

/// Trait to interpret math data structures of other math libs as their analogs in nalgebra to use operations of nalgebra.
pub mod as_native;
#[ cfg( any( feature = "nalgebra_ops", feature = "default_ops" ) ) ]
/// Use nalgebra's operations.
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
  pub use super::as_native::exposed::*;
  #[ cfg( any( feature = "nalgebra_ops", feature = "default_ops" ) ) ]
  pub use super::ops::exposed::*;
  pub use super::x2::exposed::*;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use super::as_native::prelude::*;
  #[ cfg( any( feature = "nalgebra_ops", feature = "default_ops" ) ) ]
  pub use super::ops::prelude::*;
  pub use super::x2::prelude::*;
}
