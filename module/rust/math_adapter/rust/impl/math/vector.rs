
/// Basics.
pub mod x2;

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::x2::exposed::*;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use super::x2::prelude::*;
}
