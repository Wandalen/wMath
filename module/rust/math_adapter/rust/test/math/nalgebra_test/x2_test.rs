#![ allow( unused_imports ) ]

use wtest_basic::*;
use core::mem::size_of;
use num_traits::cast::cast;
use math_adapter::prelude::*;
use math_adapter::X2;
use crate::test_tools::*;
use crate::{ num };

///
/// One test should be ordinary to exclude possibility of problems with macro.
///

fn _basic_test()
{
  type T = i8;

  /* test.case = "size"; */
  {
    assert_eq!( size_of::< nalgebra::Vector2::< T > >(), size_of::< ( T, T ) >() );
    assert_eq!( size_of::< nalgebra::Vector2::< T > >(), size_of::< [ T ; 2 ] >() );
    assert_eq!( size_of::< nalgebra::Vector2::< T > >(), 2 );
  }

  /* test.case = "value of elements"; */
  {
    let got = nalgebra::Vector2::< T >::make( 1, 2 );

    assert_eq!( got.x, 1 );
    assert_eq!( got.y, 2 );
    assert_eq!( got._0(), 1 );
    assert_eq!( got._1(), 2 );

    // tools::inspect_type_of!( got );
    // tools::inspect_type_of!( got.x );

  }

  /* making a new using the module */
  {
    let got = math_adapter::nalgebra::Vector2::< T >::make( 1, 2 );
    let exp = nalgebra::Vector2::< T >::new( 1, 2 );
    assert_eq!( got, exp );
  }

}

///
/// Parametrized test.
///

fn _canonical_test()
{

  math_adapter::for_each_number!( crate::macro_foreign_x2::macro_test_foreign_x2_number where @PREFIX( nalgebra::Vector2, x, y ) );
  math_adapter::for_each_number!( crate::macro_foreign_x2::macro_test_foreign_x2_number where @PREFIX( nalgebra::geometry::Point2, x, y ) );

//   crate::macro_foreign_x2::macro_test_foreign_x2_number!( nalgebra::Vector2, x, y ; i8, i16, i32, i64, i128 );
//   crate::macro_foreign_x2::macro_test_foreign_x2_number!( nalgebra::Vector2, x, y ; u8, u16, u32, u64, u128 );
//   crate::macro_foreign_x2::macro_test_foreign_x2_number!( nalgebra::Vector2, x, y ; f32, f64 );
//
//   crate::macro_foreign_x2::macro_test_foreign_x2_number!( nalgebra::geometry::Point2, x, y ; i8, i16, i32, i64, i128 );
//   crate::macro_foreign_x2::macro_test_foreign_x2_number!( nalgebra::geometry::Point2, x, y ; u8, u16, u32, u64, u128 );
//   crate::macro_foreign_x2::macro_test_foreign_x2_number!( nalgebra::geometry::Point2, x, y ; f32, f64 );

  /* zzz : use callback instead? */

  // trace_macros!( true );
  // macro_test_foreign_x2_number!( nalgebra::Vector2, x, y ; i8 );
  // trace_macros!( false );

}

///
/// Tests for X2 conversion function. Names are implementation-specific. .
///

#[ test ]
fn _convertion_as_specific_test()
{
  type T = i8;

  crate::macro_foreign_x2::macro_test_foreign_x2_as_specific!( nalgebra::Vector2, nalgebra ; T );

}

//

test_suite!
{
  basic_test,
  canonical_test,
  convertion_as_specific_test
}
