#![ allow( unused_imports ) ]

use test_tools::*;
use math_adapter::prelude::*;
use math_adapter::
{
  X2,
  X3,
  X4,
};

tests_impls!
{
  ///
  /// Tests for X2 conversion function as clone_as_foreign, as_foreign, as_foreign_mut .
  ///

  fn x2_convertion_foreign()
  {
    type T = i8;
    crate::macro_foreign_x2::macro_test_foreign_x2_as_foreign!( cgmath::Vector2 ; T );
  }

  ///
  /// Operations with dereferencing.
  ///

  fn x2_operation_deref()
  {
    type T = i8;
    crate::macro_foreign_x2::macro_test_foreign_x2_operation_deref!( cgmath::Vector2 ; T );
  }

  ///
  /// Tests for X3 conversion function as clone_as_foreign, as_foreign, as_foreign_mut .
  ///

  fn x3_convertion_foreign()
  {
    type T = i8;
    crate::macro_foreign_x3::macro_test_foreign_x3_as_foreign!( cgmath::Vector3 ; T );
  }

  ///
  /// Operations with dereferencing.
  ///

  fn x3_operation_deref()
  {
    type T = i8;
    crate::macro_foreign_x3::macro_test_foreign_x3_operation_deref!( cgmath::Vector3 ; T );
  }

    ///
    /// Tests for X3 conversion function as clone_as_foreign, as_foreign, as_foreign_mut .
    ///

  fn x4_convertion_foreign()
  {
    type T = i8;
    crate::macro_foreign_x4::macro_test_foreign_x4_as_foreign!( cgmath::Vector4 ; T );
  }

  ///
  /// Operations with dereferencing.
  ///

  fn x4_operation_deref()
  {
    type T = i8;
    crate::macro_foreign_x4::macro_test_foreign_x4_operation_deref!( cgmath::Vector4 ; T );
  }
}

//

tests_index!
{
  x2_convertion_foreign,
  x2_operation_deref,
  x3_convertion_foreign,
  x3_operation_deref,
  x4_convertion_foreign,
  x4_operation_deref,
}
