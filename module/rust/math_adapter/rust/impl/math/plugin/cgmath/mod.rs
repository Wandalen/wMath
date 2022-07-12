/// Internal namespace.
pub( crate ) mod private
{

  /// X2 Vector of cgmath
  pub type X2< Scalar > = cgmath::Vector2< Scalar >;

}

/// Trait to interpret math data structures of other math libs as their analogs in cgmath to use operations of cgmath.
pub mod as_foreign;
#[
  cfg( all
  (
    not( feature = "nalgebra_ops" ),
    not( all( feature = "default_ops", feature = "nalgebra" ) ),
    any( feature = "default_ops", feature = "cgmath_ops" ),
  ))
]
/// Use cgmath's operations.
pub mod ops;
/// Implement interfaces for objects of the math library.
pub mod x2;

/// Own namespace of the module.
pub mod protected
{
  pub use super::exposed::*;
  // use super::internal as i;
  pub use super::private::X2;
  pub use cgmath::*;
}

pub use protected::*;

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use super::as_foreign::exposed::*;
  #[
    cfg( all
    (
      not( feature = "nalgebra_ops" ),
      not( all( feature = "default_ops", feature = "nalgebra" ) ),
      any( feature = "default_ops", feature = "cgmath_ops" ),
    ))
  ]
  pub use super::ops::exposed::*;
  pub use super::x2::exposed::*;
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use crate::dependency::cgmath::prelude::*;
  pub use super::as_foreign::prelude::*;
  #[
    cfg( all
    (
      not( feature = "nalgebra_ops" ),
      not( all( feature = "default_ops", feature = "nalgebra" ) ),
      any( feature = "default_ops", feature = "cgmath_ops" ),
    ))
  ]
  pub use super::ops::prelude::*;
  pub use super::x2::prelude::*;
}

/* zzz : implement macro mod_adopt? */
// mod_adopt!
// {
//
//   /// Trait to interpret math data structures of other math libs as their analogs in cgmath to use operations of cgmath.
//   mod as_foreign;
//
//   #[
//     cfg( all
//     (
//       not( feature = "nalgebra_ops" ),
//       not( all( feature = "default_ops", feature = "nalgebra" ) ),
//       any( feature = "default_ops", feature = "cgmath_ops" ),
//     ))
//   ]
//   /// Use cgmath's operations.
//   mod ops;
//
//   /// Implement interfaces for objects of the math library.
//   mod x2;
//
//   own X2;
//
// }