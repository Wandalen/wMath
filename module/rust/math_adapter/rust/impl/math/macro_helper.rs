#![ allow( unused_macros ) ]
#![ allow( unused_imports ) ]

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
      $crate::for_each!
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
      $crate::for_each!
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
      $crate::for_each!
      (
        $Callback where
        $( $( $Args )* )?
        @EACH i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64
      );
    };

  }

  //

  // pub use as_path;
  // pub use as_tts;
  // pub use identity;
  pub use for_each_int;
  pub use for_each_float;
  pub use for_each_number;

}

pub use internal::*;
