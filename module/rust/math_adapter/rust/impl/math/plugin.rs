
#[cfg( feature = "winit" )]
/// Implement adapters for `winit`.
pub mod winit;
#[cfg( feature = "cgmath" )]
/// Implement adapters for `cgmath`.
pub mod cgmath;

/// Exposed namespace of the module.
pub mod exposed
{
  #[cfg( feature = "winit" )]
  pub use super::winit::exposed::*;
  #[cfg( feature = "cgmath" )]
  pub use super::cgmath::exposed::*;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  #[cfg( feature = "winit" )]
  pub use super::winit::prelude::*;
  #[cfg( feature = "cgmath" )]
  pub use super::cgmath::prelude::*;
}
