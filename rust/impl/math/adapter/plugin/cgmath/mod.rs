
//!
//! Implement adapters for `cgmath`.
//!

/// Internal namespace.
pub( crate ) mod private
{

  /// X2 Vector of cgmath
  pub type X2< Scalar > = cgmath::Vector2< Scalar >;

  /// X3 Vector of cgmath
  pub type X3< Scalar > = cgmath::Vector3< Scalar >;

  /// X4 Vector of cgmath
  pub type X4< Scalar > = cgmath::Vector4< Scalar >;

}

crate::mod_interface!
{
  // #![ debug ]


  /// Trait to interpret math data structures of other math libs as their analogs in cgmath to use operations of cgmath.
  layer as_foreign;

  #[ cfg( cgmath_ops ) ]
  /// Use cgmath's operations.
  layer ops;

  /// Implement interfaces for objects of the math library.
  layer x2;
  /// Implement 3 dimension interfaces for objects of the math library.
  layer x3;
  /// Implement 4 dimension interfaces for objects of the math library.
  layer x4;

  protected use X2;
  protected use X3;
  protected use X4;
  protected use ::cgmath::*;
  prelude use ::cgmath::Array;

  // #[ cfg( cgmath_ops ) ]
  // exposed use X2;

}
