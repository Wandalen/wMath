#![ allow( unused_macros ) ]
#![ allow( unused_imports ) ]

/// Internal namespace.
pub mod internal
{

//   #[macro_export]
//   macro_rules! apply_simple
//   {
//
//     (
//       $Callback : path, $( $Each : tt ),*
//     ) =>
//     {
//       $(
//         $Callback!
//         (
//           $Each
//         );
//       )*
//     };
//
//   }

  ///
  /// Apply callback with prefix and postfix.
  ///

  #[macro_export]
  macro_rules! apply
  {

    // -- non-named

    (
      $Callback : path, $( $Each : tt ),*
    ) =>
    {
      $(
        $Callback!
        (
          $Each
        );
      )*
    };

    // -- named with parentheses

    (
      $Callback : path
      where
        @EACH( $( $Each : tt ),* )
    ) =>
    {
      $(
        $Callback!
        (
          ( $Each )
        );
      )*
    };

    (
      $Callback : path
      where
        @PREFIX $Prefix : tt
        @EACH( $( $Each : tt ),* )
    ) =>
    {
      $(
        $Callback!
        (
          $Prefix
          ( $Each )
        );
      )*
    };

    (
      $Callback : path
      where
        @POSTFIX $Postfix : tt
        @EACH( $( $Each : tt ),* )
    ) =>
    {
      $(
        $Callback!
        (
          ( $Each )
          $Postfix
        );
      )*
    };

    (
      $Callback : path
      where
        @PREFIX $Prefix : tt
        @POSTFIX $Postfix : tt
        @EACH( $( $Each : tt ),* )
    ) =>
    {
      $(
        $Callback!
        (
          $Prefix
          ( $Each )
          $Postfix
        );
      )*
    };

    // -- named without parentheses

    (
      $Callback : path
      where
        @EACH $( $Each : tt ),*
    ) =>
    {
      $(
        $Callback!
        (
          $Each
        );
      )*
    };

    (
      $Callback : path
      where
        @PREFIX $Prefix : tt
        @EACH $( $Each : tt ),*
    ) =>
    {
      $(
        $Callback!
        (
          $Prefix
          $Each
        );
      )*
    };

    (
      $Callback : path
      where
        @POSTFIX $Postfix : tt
        @EACH $( $Each : tt ),*
    ) =>
    {
      $(
        $Callback!
        (
          $Each
          $Postfix
        );
      )*
    };

    (
      $Callback : path
      where
        @PREFIX $Prefix : tt
        @POSTFIX $Postfix : tt
        @EACH $( $Each : tt ),*
    ) =>
    {
      $(
        $Callback!
        (
          $Prefix
          $Each
          $Postfix
        );
      )*
    };

    // -- with @ARGS

    (
      $Callback : path =>
      @ARGS( @PREFIX $Prefix : tt @POSTFIX $Postfix : tt  )
      @EACH( $( $Each : tt ),* )
    ) =>
    {
      $(
        $Callback!
        (
          $Prefix
          $Each
          $Postfix
        );
      )*
    };

    (
      $Callback : path =>
      @ARGS( @PREFIX $Prefix : tt )
      @EACH( $( $Each : tt ),* )
    ) =>
    {
      $(
        $Callback!
        (
          $Prefix
          $Each
        );
      )*
    };

    (
      $Callback : path =>
      @ARGS( @POSTFIX $Postfix : tt  )
      @EACH( $( $Each : tt ),* )
    ) =>
    {
      $(
        $Callback!
        (
          $Each
          $Postfix
        );
      )*
    };

    (
      $Callback : path =>
      @ARGS()
      @EACH( $( $Each : tt ),* )
    ) =>
    {
      $(
        $Callback!
        (
          $Each
        );
      )*
    };

  }

  ///
  /// Apply callback to each integer : i8, i16, i32, i64, i128, u8, u16, u32, u64, u128.
  ///

  #[macro_export]
  macro_rules! for_each_int
  {

    (
      $Callback : path
      $( where $( $Args : tt )* )?
    ) =>
    {
      $crate::apply!
      (
        $Callback =>
        @ARGS( $( $( $Args )* )? )
        @EACH( i8, i16, i32, i64, i128, u8, u16, u32, u64, u128 )
      );
    };

  }

  ///
  /// Apply callback to each float : f32, f64.
  ///

  #[macro_export]
  macro_rules! for_each_float
  {

    (
      $Callback : path
      $( where $( $Args : tt )* )?
    ) =>
    {
      $crate::apply!
      (
        $Callback =>
        @ARGS( $( $( $Args )* )? )
        @EACH( f32, f64 )
      );
    };

  }

  ///
  /// Apply callback to each number : i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64.
  ///

  #[macro_export]
  macro_rules! for_each_number
  {

    (
      $Callback : path
      $( where $( $Args : tt )* )?
    ) =>
    {
      $crate::apply!
      (
        $Callback =>
        @ARGS( $( $( $Args )* )? )
        @EACH( i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64 )
      );
    };

  }

  //

  pub use apply;
  pub use for_each_int;
  pub use for_each_float;
  pub use for_each_number;

}

pub use internal::*;
