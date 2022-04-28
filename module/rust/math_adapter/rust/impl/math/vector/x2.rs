/// Internal namespace.
pub mod internal
{
  use core::fmt::Debug;
  use core::hash::Hash;
  use core::ops::Add;
  use crate::ScalarInterface;
  // use crate::prelude::*;

  include!( "./x2_interface.rs" );
  include!( "./x2_struct.rs" );

  //

}

/// Exposed namespace of the module.
pub mod exposed
{
  use super::internal as i;
  pub use i::x2;
  pub use i::x2_interface;
  pub use i::x2_canonical_interface;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  use super::internal as i;
  pub use i::x2_interface;
  pub use i::x2_canonical_interface;
}
