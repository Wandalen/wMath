//!
//! Define several traits like NanLikeInterface.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  ///
  /// Implement check is it nan and constructor with NAN value.
  ///

  pub trait NanLikeInterface
  {
    /// Construct NAN-like. If the type does not have NAN value in codomain of the type it should return default value.
    fn make_nan_like() -> Self;
    /// Is current value NAN? Always false if codomain of the type does not have NAN value.
    fn is_nan( &self ) -> bool;
  }

  //

  macro_rules! impl_nan_like_for_integer
  {
    (
      $( $Args : tt )*
    ) =>
    {
      impl NanLikeInterface for $( $Args )*
      {
        #[ inline ]
        fn make_nan_like() -> Self
        {
          0
        }
        #[ inline ]
        fn is_nan( &self ) -> bool
        {
          false
        }
      }
    };
  }

  //

  macro_rules! impl_nan_like_for_float
  {
    (
      $( $Args : tt )*
    ) =>
    {
      impl NanLikeInterface for $( $Args )*
      {
        #[ inline ]
        fn make_nan_like() -> Self
        {
          < $( $Args )* >::NAN
        }
        #[ inline ]
        fn is_nan( &self ) -> bool
        {
          *self == < $( $Args )* >::NAN
        }
      }
    };
  }

  for_each_int!( impl_nan_like_for_integer );
  for_each_float!( impl_nan_like_for_float );

}

crate::mod_interface!
{
  prelude use NanLikeInterface;
}
