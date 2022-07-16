#![ allow( unused_macros ) ]
#![ allow( unused_imports ) ]

/// Internal namespace.
pub( crate ) mod private
{

  ///
  /// Apply callback to each integer : isize, i8, i16, i32, i64, i128m, usize, u8, u16, u32, u64, u128.
  ///

  #[ macro_export ]
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
        @Each isize i8 i16 i32 i64 i128 usize u8 u16 u32 u64 u128
      );
    };

  }

  ///
  /// Apply callback to each float : f32, f64.
  ///

  #[ macro_export ]
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
        @Each f32 f64
      );
    };

  }

  ///
  /// Apply callback to each number : isize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64.
  ///

  #[ macro_export ]
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
        @Each isize i8 i16 i32 i64 i128 usize u8 u16 u32 u64 u128 f32 f64
      );
    };

  }

  //

  pub use for_each_int;
  pub use for_each_float;
  pub use for_each_number;

}

// pub use private::*;
crate::mod_interface!
{
  orphan use
  {
    for_each_int,
    for_each_float,
    for_each_number,
  };
}
