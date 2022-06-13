/// Internal namespace.
pub( crate ) mod internal
{

  // pub trait i32x1
  // {
  // }

}

/// Exposed namespace of the module.
pub mod exposed
{
  // use super::internal as i;

}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  // use super::internal as i;

}
