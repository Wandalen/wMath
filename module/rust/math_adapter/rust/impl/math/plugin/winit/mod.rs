/// Internal namespace.
pub( crate ) mod private
{
  // use core::mem::size_of;
  // use crate::prelude::*;
  // use crate::X2;
  // use crate::ScalarInterface;

  // include!( "./x2.rs" );
  // impl_x2_for!( winit::dpi::PhysicalSize< Scalar > );

}

/// Implement X2 interfaces for vectors of the math lib.
pub mod x2;

/// Own namespace of the module.
pub mod protected
{
  // // use super::internal as i;
  // pub use super::private::X2;
}

pub use protected::*;

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::x2::exposed::*;
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::x2::prelude::*;
}
