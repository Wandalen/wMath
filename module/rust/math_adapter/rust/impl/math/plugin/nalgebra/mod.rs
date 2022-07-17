// /// Internal namespace.
// pub( crate ) mod private
// {
//   /// X2 Vector of nalgebra
//   pub type X2< Scalar > = nalgebra::Vector2< Scalar >;
//
// }
//
// pub mod as_foreign;
// #[ cfg( any( feature = "nalgebra_ops", feature = "default_ops" ) ) ]
// pub mod ops;
// pub mod x2;
//
// /// Own namespace of the module.
// pub mod protected
// {
//   // use super::internal as i;
//   pub use super::private::X2;
//   pub use nalgebra::*;
// }
//
// pub use protected::*;
//
// /// Orphan namespace of the module.
// pub mod orphan
// {
//   pub use super::exposed::*;
// }
//
// /// Exposed namespace of the module.
// pub mod exposed
// {
//   pub use super::as_foreign::exposed::*;
//   #[ cfg( any( feature = "nalgebra_ops", feature = "default_ops" ) ) ]
//   pub use super::ops::exposed::*;
//   pub use super::x2::exposed::*;
// }
//
// pub use exposed::*;
//
// /// Prelude to use essentials: `use my_module::prelude::*`.
// pub mod prelude
// {
//   pub use super::as_foreign::prelude::*;
//   #[ cfg( any( feature = "nalgebra_ops", feature = "default_ops" ) ) ]
//   pub use super::ops::prelude::*;
//   pub use super::x2::prelude::*;
//   // use crate::dependency::nalgebra::prelude::*;
// }

//!
//! Implement adapters for `nalgebra`.
//!

/// Internal namespace.
pub( crate ) mod private
{

  /// X2 Vector of nalgebra
  pub type X2< Scalar > = ::nalgebra::Vector2< Scalar >;

}

//

crate::mod_interface!
{

  layer as_foreign;
  #[ cfg( any( feature = "nalgebra_ops", feature = "default_ops" ) ) ]
  layer ops;
  layer x2;

  protected use X2;
  protected use ::nalgebra::*;

}