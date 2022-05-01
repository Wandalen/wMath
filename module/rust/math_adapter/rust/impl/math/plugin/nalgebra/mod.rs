/// Internal namespace.
pub mod internal
{
  use core::mem::size_of;
  use crate::prelude::*;
  use crate::X2;
  use crate::ScalarInterface;

  include!( "./X2.rs" );
  impl_x2_for!( nalgebra::Vector2< Scalar > );

}

/// Trait to interpret math data structures of other math libs as their analogs in nalgebra to use operations of nalgebra.
pub mod as_nalgebra;

/// Exposed namespace of the module.
pub mod exposed
{
  // use super::internal as i;
  pub use super::as_nalgebra::exposed::*;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
  pub use super::as_nalgebra::prelude::*;
}
