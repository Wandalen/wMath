
#[cfg( feature = "winit" )]
/// Implement adapters for `winit`.
pub mod winit;

/// Exposed namespace of the module.
pub mod exposed
{
  #[cfg( feature = "winit" )]
  pub use super::winit::exposed::*;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  #[cfg( feature = "winit" )]
  pub use super::winit::prelude::*;
}
