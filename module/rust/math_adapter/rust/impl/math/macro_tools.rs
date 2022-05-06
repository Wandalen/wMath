#![ allow( unused_macros ) ]
#![ allow( unused_imports ) ]

/* zzz : qqq : move to module::macro_for_each
- discuss list of features of the module
use module::macro_for_each in module::macro_tools
*/

/// Internal namespace.
pub mod internal
{

  // /// Reinterpret tokens as path.
  // #[macro_export]
  // macro_rules! as_path { ( $src : path ) => { $src } }
  // /// Reinterpret tokens as tts.
  // #[macro_export]
  // macro_rules! as_tts { ( $( $src : tt )* ) => { $( $src )* } }
  // /// Macro which returns its input as
  // #[macro_export]
  // macro_rules! identity
  // {
  //   (
  //     ( $Src : tt ),*
  //   ) =>
  //   {
  //     ( $Src ),*
  //   };
  // }

  ///
  /// Unwrap braces of token tree and pass its content to the passed callback. If token tree in not braced then it passed to callback as is.
  ///
  /// # Function-style sample
  /// ```rust
  /// let ( a, b, c ) = ( 1, 2, 3 );
  /// math_adapter::braces_unwrap!( dbg, { a, b, c } );
  /// // generates :
  /// // dbg!( a, b, c );
  /// math_adapter::braces_unwrap!( dbg, a, b, c );
  /// // generates :
  /// // dbg!( a, b, c );
  /// ```
  ///
  /// # Map-style sample
  /// ```
  /// let ( prefix, a, b, c, postfix ) = ( "prefix", 1, 2, 3, "postfix" );
  /// math_adapter::braces_unwrap!
  /// (
  ///   dbg where
  ///   @PREFIX{ prefix, }
  ///   @POSTFIX{ postfix }
  ///   @SRC{ { a, b, c, } }
  /// );
  /// // generates :
  /// // dbg!( prefix, a, b, c, psotfix );
  /// math_adapter::braces_unwrap!
  /// (
  ///   dbg where
  ///   @PREFIX{ prefix, }
  ///   @POSTFIX{ postfix }
  ///   @SRC{ a, b, c, }
  /// );
  /// // generates :
  /// // dbg!( prefix, a, b, c, psotfix );
  /// ```
  ///

  #[macro_export]
  macro_rules! braces_unwrap
  {

    // function-style

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

    // map-style

    (
      $Callback : path where
      @SRC{ { $( $Src : tt )* } }
    )
    =>
    {
      $Callback!
      (
        $( $Src )*
      );
    };
    (
      $Callback : path where
      @SRC{ $( $Src : tt )* }
    )
    =>
    {
      $Callback!
      (
        $( $Src )*
      );
    };

    // with prefix and psotfix

    /* 0 */
    (
      $Callback : path where
      @PREFIX{ { $( $Prefix : tt )* } }
      @POSTFIX{ { $( $Postfix : tt )* } }
      @SRC{ { $( $Src : tt )* } }
    )
    =>
    {
      $Callback!
      (
        $( $Prefix )* $( $Src )* $( $Postfix )*
      );
    };
    /* 1 */
    (
      $Callback : path where
      @PREFIX{ { $( $Prefix : tt )* } }
      @POSTFIX{ { $( $Postfix : tt )* } }
      @SRC{ $( $Src : tt )* }
    )
    =>
    {
      $Callback!
      (
        $( $Prefix )* $( $Src )* $( $Postfix )*
      );
    };
    /* 2 */
    (
      $Callback : path where
      @PREFIX{ { $( $Prefix : tt )* } }
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
    /* 3 */
    (
      $Callback : path where
      @PREFIX{ { $( $Prefix : tt )* } }
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
    /* 4 */
    (
      $Callback : path where
      @PREFIX{ $( $Prefix : tt )* }
      @POSTFIX{ { $( $Postfix : tt )* } }
      @SRC{ { $( $Src : tt )* } }
    )
    =>
    {
      $Callback!
      (
        $( $Prefix )* $( $Src )* $( $Postfix )*
      );
    };
    /* 5 */
    (
      $Callback : path where
      @PREFIX{ $( $Prefix : tt )* }
      @POSTFIX{ { $( $Postfix : tt )* } }
      @SRC{ $( $Src : tt )* }
    )
    =>
    {
      $Callback!
      (
        $( $Prefix )* $( $Src )* $( $Postfix )*
      );
    };
    /* 6 */
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
    /* 7 */
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

    // with prefix

    /* 0 */
    (
      $Callback : path where
      @PREFIX{ { $( $Prefix : tt )* } }
      @SRC{ { $( $Src : tt )* } }
    )
    =>
    {
      $Callback!
      (
        $( $Prefix )* $( $Src )*
      );
    };
    /* 1 */
    (
      $Callback : path where
      @PREFIX{ { $( $Prefix : tt )* } }
      @SRC{ $( $Src : tt )* }
    )
    =>
    {
      $Callback!
      (
        $( $Prefix )* $( $Src )*
      );
    };
    /* 2 */
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
    /* 3 */
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

    /* 0 */
    (
      $Callback : path where
      @POSTFIX{ { $( $Postfix : tt )* } }
      @SRC{ { $( $Src : tt )* } }
    )
    =>
    {
      $Callback!
      (
        $( $Src )* $( $Postfix )*
      );
    };
    /* 1 */
    (
      $Callback : path where
      @POSTFIX{ { $( $Postfix : tt )* } }
      @SRC{ $( $Src : tt )* }
    )
    =>
    {
      $Callback!
      (
        $( $Src )* $( $Postfix )*
      );
    };
    /* 2 */
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
    /* 3 */
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

  }

  /// Macro which returns its input as is.

  #[macro_export]
  macro_rules! identity
  {
    (
      ( $Src : tt ),*
    ) =>
    {
      ( $Src ),*
    };
  }

  ///
  /// Apply callback with prefix and postfix.
  ///
  /// # Function-style call
  /// ```rust
  /// math_adapter::for_each!( dbg, "a", "b", "c" );
  /// ```
  /// Generates:
  /// ```rust
  /// dbg!( "a" );
  /// dbg!( "b" );
  /// dbg!( "b" );
  /// ```
  ///
  /// # Map-style call
  /// ```rust
  /// math_adapter::for_each!
  /// (
  ///   dbg where
  ///   // xxx
  ///   // @PREFIX prefix
  ///   // @POSTFIX postfix
  ///   @EACH "a", "b", "c"
  /// );
  /// ```
  /// Generates:
  /// ```rust
  /// dbg!( "a" );
  /// dbg!( "b" );
  /// dbg!( "b" );
  /// ```
  ///

  #[macro_export]
  macro_rules! for_each
  {

    // -- calbackless

    // (
    //   $Prefix : tt
    //   @EACH{ $( $Each : tt ),* $(,)? }
    //   $Postfix : tt
    // ) =>
    // {
    //   $(
    //     $crate::braces_unwrap!
    //     (
    //       $crate::identity where
    //       @PREFIX{ $Prefix }
    //       @POSTFIX{ $Postfix }
    //       @SRC{ $Each }
    //     );
    //   )*
    // };

    // -- function-style

    (
      $Callback : path, $( $Each : tt ),* $(,)?
    ) =>
    {
      $(
        $crate::braces_unwrap!( $Callback, $Each );
      )*
    };

    // -- map-style

    (
      $Callback : path where
      @EACH $( $Each : tt ),* $(,)?
    ) =>
    {
      $(
        $crate::braces_unwrap!( $Callback, $Each );
      )*
    };

    (
      $Callback : path
      where
        @PREFIX $Prefix : tt
        @POSTFIX $Postfix : tt
        @EACH $( $Each : tt ),* $(,)?
    ) =>
    {
      $(
        $crate::braces_unwrap!
        (
          $Callback where
          @PREFIX{ $Prefix }
          @POSTFIX{ $Postfix }
          @SRC{ $Each }
        );
      )*
    };

    (
      $Callback : path where
      @PREFIX $Prefix : tt
      @EACH $( $Each : tt ),* $(,)?
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
      @EACH $( $Each : tt ),* $(,)?
    ) =>
    {
      $(
        $crate::braces_unwrap!
        (
          $Callback where
          @POSTFIX{ $Postfix }
          @SRC{ $Each }
        );
      )*
    };

  }

  //

  pub use braces_unwrap;
  pub use for_each;

}

pub use internal::*;
