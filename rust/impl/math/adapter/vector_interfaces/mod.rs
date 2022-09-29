//!
//! Macro to implement XnNominalInterface, XnBasicInterface, XnCanonicalInterface traits for an arbitrary type.
//!

/// Internal namespace.
pub( crate ) mod private
{
  pub use derive_vector_interfaces::VectorInterfaces;
}

crate::mod_interface!
{
  prelude use VectorInterfaces;
}
