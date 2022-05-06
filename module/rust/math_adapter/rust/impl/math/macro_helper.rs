#![ allow( unused_macros ) ]
#![ allow( unused_imports ) ]

/* zzz : qqq : move to module::macro_apply
- discuss list of features of the module
use module::macro_apply in module::macro_tools
*/

/// Internal namespace.
pub mod internal
{

  /// Reinterpret tokens as path.
  #[macro_export]
  macro_rules! as_path { ( $src : path ) => { $src } }
  /// Reinterpret tokens as tts.
  #[macro_export]
  macro_rules! as_tts { ( $( $src : tt )* ) => { $( $src )* } }

  /// Unwrap parentheses.
  #[macro_export]
  macro_rules! parentheses_unwrap
  {
    ( $Callback : path, ( $( $src : tt )* ) )
    =>
    {
      $Callback!
      (
        $( $src )*
      )
    };
    ( $Callback : path, $( $src : tt )* )
    =>
    {
      $Callback!
      (
        $( $src )*
      )
    };
  }

  /// Unwrap braces.
  #[macro_export]
  macro_rules! braces_unwrap
  {

    // unnamed

    ( $Callback : path, { $( $Src : tt )* } )
    =>
    {
      $Callback!
      (
        $( $Src )*
      );
    };
    ( $Callback : path, $( $Src : tt )* )
    =>
    {
      $Callback!
      (
        $( $Src )*
      );
    };

    // with prefix

    (
      $Callback : path where
      @PREFIX{ $( $Prefix : tt )* }
      @SRC{ { $( $Src : tt )* } }
    )
    =>
    {
      $Callback!
      (
        $( $Prefix )* $( $Src )*
      );
    };
    (
      $Callback : path where
      @PREFIX{ $( $Prefix : tt )* }
      @SRC{ $( $Src : tt )* }
    )
    =>
    {
      $Callback!
      (
        $( $Prefix )* $( $Src )*
      );
    };

    // with postfix

    (
      $Callback : path where
      @POSTFIX{ $( $Postfix : tt )* }
      @SRC{ { $( $Src : tt )* } }
    )
    =>
    {
      $Callback!
      (
        $( $Src )* $( $Postfix )*
      );
    };
    (
      $Callback : path where
      @POSTFIX{ $( $Postfix : tt )* }
      @SRC{ $( $Src : tt )* }
    )
    =>
    {
      $Callback!
      (
        $( $Src )* $( $Postfix )*
      );
    };

    // with prefix and psotfix

    (
      $Callback : path where
      @PREFIX{ $( $Prefix : tt )* }
      @POSTFIX{ $( $Postfix : tt )* }
      @SRC{ { $( $Src : tt )* } }
    )
    =>
    {
      $Callback!
      (
        $( $Prefix )* $( $Src )* $( $Postfix )*
      );
    };
    (
      $Callback : path where
      @PREFIX{ $( $Prefix : tt )* }
      @POSTFIX{ $( $Postfix : tt )* }
      @SRC{ $( $Src : tt )* }
    )
    =>
    {
      $Callback!
      (
        $( $Prefix )* $( $Src )* $( $Postfix )*
      );
    };

  }

  ///
  /// Apply callback with prefix and postfix.
  ///
  /// # Non-named call
  /// ```rust
  /// math_adapter::apply!( dbg, "a", "b", "c" );
  /// ```
  /// Generate:
  /// ```rust
  /// dbg!( "a" );
  /// dbg!( "b" );
  /// dbg!( "b" );
  /// ```
  ///
  /// # Named call
  /// ```rust
  /// math_adapter::apply!
  /// (
  ///   dbg where
  ///   // xxx
  ///   // @PREFIX prefix
  ///   // @POSTFIX postfix
  ///   @EACH "a", "b", "c"
  /// );
  /// ```
  /// Generate:
  /// ```rust
  /// dbg!( "a" );
  /// dbg!( "b" );
  /// dbg!( "b" );
  /// ```
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
        $crate::braces_unwrap!( $Callback, $Each );
      )*
    };

    // -- named without parentheses

    (
      $Callback : path where
      @EACH $( $Each : tt ),*
    ) =>
    {
      $(
        $crate::braces_unwrap!( $Callback, $Each );
      )*
    };

    // (
    //   $Callback : path where
    //   @PREFIX $Prefix : tt
    //   @EACH $( $Each : tt ),*
    // ) =>
    // {
    //   $(
    //     $Callback!
    //     (
    //       $Prefix
    //       $Each
    //     );
    //   )*
    // };

    (
      $Callback : path where
      @PREFIX $Prefix : tt
      @EACH $( $Each : tt ),*
    ) =>
    {
      $(
        $crate::braces_unwrap!
        (
          $Callback where
          @PREFIX{ $Prefix }
          @SRC{ $Each }
        );
      )*
    };

    (
      $Callback : path where
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
        $Callback where
        $( $( $Args )* )?
        @EACH i8, i16, i32, i64, i128, u8, u16, u32, u64, u128
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
        $Callback where
        $( $( $Args )* )?
        @EACH f32, f64
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
        $Callback where
        $( $( $Args )* )?
        @EACH i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64
      );
    };

  }

  //

  pub use as_path;
  pub use as_tts;
  pub use apply;
  pub use for_each_int;
  pub use for_each_float;
  // pub use for_each_float;
  pub use for_each_number;

}

pub use internal::*;
