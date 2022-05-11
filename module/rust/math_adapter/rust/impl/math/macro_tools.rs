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
  /// Macro which returns its input as
  #[macro_export]
  macro_rules! identity
  {
    (
      $( $Src : tt )*
    )
    =>
    {
      $( $Src )*
    };
  }

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

  ///
  /// Apply macro for each element of a list.
  ///
  /// Macros $Callback is called for each element of passed list, optinally passing prefix $Prefix as first argument(s) and postfix $Postfix as the last argument(s).
  /// Macros could be invoked in either function call style or map call style. Prefix and postfix could be passed only in map call style.
  /// In map call style after passing path to macro pass keyword `where` and options in format : `@KEY Value`.
  ///
  /// In some cases same code may be generated without callback macro, just using prefix and postfix.
  /// That's why $Callback is also optional.
  /// To invoke `for_each` without callback use map call style omitting path to callback and keyword `where`.
  ///
  /// # Sample : function-style call
  ///
  /// Macro `for_each` may be called either in function-style way or in map-style way.
  /// Pass name of macro to apply to elements as the first arguments and elements after the macro name.
  /// Use comma as delimeter.
  ///
  /// ```rust
  /// math_adapter::for_each!( dbg, "a", "b", "c" );
  /// ```
  /// Generates:
  /// ```rust
  /// math_adapter::for_each!( dbg, "a", "b", "c" );
  /// // generates
  /// dbg!( "a" );
  /// dbg!( "b" );
  /// dbg!( "c" );
  /// ```
  ///
  /// # Sample : map-style call
  ///
  /// Macro `for_each` may be called either in function-style way or in map-style way.
  /// Use keys @PREFIX @POSTFIX @EACH to pass options as entries of a map.
  /// Options @PREFIX and @POSTFIX are optional and their entries could be ommited, but entry @EACH is mandatory.
  /// Order of options should always be @PREFIX, @POSTFIX, @EACH.
  ///
  /// ```rust
  /// math_adapter::for_each!
  /// {
  ///   dbg where
  ///   @PREFIX { "prefix".to_string() + }
  ///   @POSTFIX { + "postfix" }
  ///   @EACH "a" "b" "c"
  /// };
  /// // generates
  /// dbg!( "prefix".to_string() + "a" + "postfix" );
  /// dbg!( "prefix".to_string() + "b" + "postfix" );
  /// dbg!( "prefix".to_string() + "c" + "postfix" );
  /// ```
  /// Generates:
  /// ```rust
  /// dbg!( "a" );
  /// dbg!( "b" );
  /// dbg!( "b" );
  /// ```
  ///
  /// # Sample : more than single token
  ///
  /// Both prefix and postfix have to be token tree ( `tt` ). But if you need something more complex put it into braces `{ ... }`.
  /// Macros `for_each` will remove outermost braces. Braces are optional in case of prefix/postfix is a singlle token.
  ///
  /// ```
  /// math_adapter::for_each!
  /// {
  ///   dbg where
  ///   @PREFIX { "prefix".to_string() + }
  ///   @POSTFIX { + "postfix" }
  ///   @EACH { "a" + "1" } { "b" + "2" } { "c" + "3" }
  /// };
  /// // generates
  /// dbg!( "prefix".to_string() + "a" + "1" + "postfix" );
  /// dbg!( "prefix".to_string() + "b" + "2" + "postfix" );
  /// dbg!( "prefix".to_string() + "c" + "3" + "postfix" );
  /// ```
  ///
  /// # Sample : callbackless
  ///
  /// Callback macro is optinal.
  /// Use map call style and omit path to callback macro with keyword `where` to invoke `for_each` without a callback.
  ///
  /// ```
  /// math_adapter::for_each!
  /// {
  ///   @PREFIX { dbg! }
  ///   @EACH ( "a" ) ( "b" ) ( "c" )
  /// };
  /// // generates
  /// dbg!( "a" );
  /// dbg!( "b" );
  /// dbg!( "c" );
  /// ```
  ///

  #[macro_export]
  macro_rules! for_each
  {

    // -- function-style

    (
      $Callback : path, $( $Each : tt ),* $(,)?
    ) =>
    {
      $(
        $crate::braces_unwrap!( $Callback, $Each );
      )*
    };

    // -- callback-less

    (
      @PREFIX $Prefix : tt
      @POSTFIX $Postfix : tt
      @EACH $( $Each : tt )*
    ) =>
    {
      $crate::for_each!
      {
        $crate::identity where
        @PREFIX $Prefix
        @POSTFIX $Postfix
        @EACH $( $Each )*
      }
    };

    (
      @PREFIX $Prefix : tt
      @EACH $( $Each : tt )*
    ) =>
    {
      $crate::for_each!
      {
        $crate::identity where
        @PREFIX $Prefix
        @EACH $( $Each )*
      }
    };

    (
      @POSTFIX $Postfix : tt
      @EACH $( $Each : tt )*
    ) =>
    {
      $crate::for_each!
      {
        $crate::identity where
        @POSTFIX $Postfix
        @EACH $( $Each )*
      }
    };

    // -- map-style

    (
      $Callback : path where
      @EACH $( $Each : tt )*
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
        @EACH $( $Each : tt )*
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
      @EACH $( $Each : tt )*
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
      @EACH $( $Each : tt )*
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

  pub use identity;
  pub use braces_unwrap;
  pub use for_each;

}

pub use internal::*;
