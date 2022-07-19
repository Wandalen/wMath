#![ allow( unused_imports ) ]

use test_tools::*;
use math_adapter::prelude::*;
use math_adapter::X2;

tests_impls!
{
  ///
  /// Tests for X2 conversion function as clone_as_foreign, as_foreign, as_foreign_mut .
  ///

  fn convertion_foreign()
  {
    type T = i8;
    crate::macro_foreign_x2::macro_test_foreign_x2_as_foreign!( cgmath::Vector2 ; T );
  }

  ///
  /// Operations with dereferencing.
  ///

  fn operation_deref()
  {
    type T = i8;
    crate::macro_foreign_x2::macro_test_foreign_x2_operation_deref!( cgmath::Vector2 ; T );
  }
}

//

tests_index!
{
  convertion_foreign,
  operation_deref,
}
