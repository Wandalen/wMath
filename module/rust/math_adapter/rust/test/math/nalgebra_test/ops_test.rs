// #![ allow( unused_imports ) ]

use wtest_basic::*;
use crate::test_tools::*;
use math_adapter::prelude::*;
use math_adapter::X2;
// use nalgebra::Vector2 as nalgebra::Vector2;

///
/// Tests for X2 conversion function as clone_as_foreign, as_foreign, as_foreign_mut .
///

fn _convertion_foreign_test()
{
  type T = i8;

  crate::macro_x2::macro_test_x2_as_foreign!( nalgebra::Vector2 ; T );

}

///
/// Operations with dereferencing.
///

fn _operation_deref_test()
{
  type T = i8;

  crate::macro_x2::macro_test_x2_operation_deref!( nalgebra::Vector2 ; T );

}

///
/// Interoperability with cgmath.
///

fn _inter_cgmath_test()
{
  #[ cfg( feature = "cgmath" ) ]
  {

    type T = i8;

    /* test.case = "using add operator of nalgebra"; */
    {
      let src1 = cgmath::Vector2::< T >::make( 1, 2 );
      let src2 = nalgebra::Vector2::< T >::make( 3, 4 );
      let got = src1.as_nalgebra() + src2;
      let exp = nalgebra::Vector2::< T >::make( 4, 6 );
      assert_eq!( got, exp );
    }

    /* test.case = "using either `cgmath's` and `nalgebra's` implementation of sum"; */
    {
      // use cgmath::Array;
      let src = X2::< T >::make( 1, 2 );
      let got = src.as_cgmath().sum();
      assert_eq!( got, 3 );
      let got = src.as_nalgebra().sum();
      assert_eq!( got, 3 );
    }

  }
}

//

test_suite!
{
  convertion_foreign_test,
  operation_deref_test,
  inter_cgmath_test,
}

/* xxx : teach test_suite to understand directives */
