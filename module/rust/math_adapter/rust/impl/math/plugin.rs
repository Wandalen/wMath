
#[cfg( feature = "cgmath" )]
/// Implement adapters for `cgmath`.
pub mod cgmath;
#[cfg( feature = "nalgebra" )]
/// Implement adapters for `nalgebra`.
pub mod nalgebra;
#[cfg( feature = "winit" )]
/// Implement adapters for `winit`.
pub mod winit;

/// Exposed namespace of the module.
pub mod exposed
{
  #[cfg( feature = "cgmath" )]
  pub use super::cgmath::exposed::*;
  #[cfg( feature = "nalgebra" )]
  pub use super::nalgebra::exposed::*;
  #[cfg( feature = "winit" )]
  pub use super::winit::exposed::*;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  #[cfg( feature = "cgmath" )]
  pub use super::cgmath::prelude::*;
  #[cfg( feature = "nalgebra" )]
  pub use super::nalgebra::exposed::*;
  #[cfg( feature = "winit" )]
  pub use super::winit::prelude::*;
}
