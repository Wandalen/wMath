#![ allow( unused_imports ) ]

use wtest_basic::*;
use core::mem::size_of;
use num_traits::cast::cast;
use math_adapter::prelude::*;
use math_adapter::X2;
use crate::tools::*;
use crate::{ x2_with_records_test_for, val };

///
/// One test should be ordinary to exclude possibility of problems with macro.
///

fn _basic()
{
  type T = i8;

  /* test.case = "size"; */
  {
    assert_eq!( size_of::< cgmath::Vector2::< T > >(), size_of::< ( T, T ) >() );
    assert_eq!( size_of::< cgmath::Vector2::< T > >(), size_of::< [ T ; 2 ] >() );
    assert_eq!( size_of::< cgmath::Vector2::< T > >(), 2 );
  }

  /* test.case = "value of elements"; */
  {
    let got = cgmath::Vector2::< i8 >{ x : 1, y : 2 };
    assert_eq!( got.x, 1 );
    assert_eq!( got.y, 2 );
    assert_eq!( got._0(), 1 );
    assert_eq!( got._1(), 2 );
  }

  // /* test.case = "operator add"; */
  // {
  //   let src1 = cgmath::Vector2::< i8 >{ x : 1, y : 2 };
  //   let src2 = cgmath::Vector2::< i8 >{ x : 2, y : 3 };
  //   let got = src1.as_canonical() + src2.as_canonical();
  //   let exp = math_adapter::X2::< i8 >( 3, 5 );
  //   assert_eq!( got, exp );
  // }

}

//

fn _canonical_test()
{
  x2_with_records_test_for!( cgmath::Vector2, x, y ; i8, i16, i32, i64, i128 );
  x2_with_records_test_for!( cgmath::Vector2, x, y ; u8, u16, u32, u64, u128 );
  x2_with_records_test_for!( cgmath::Vector2, x, y ; f32, f64 );

  // trace_macros!( true );
  // x2_with_records_test_for!( cgmath::Vector2, x, y ; f32 );
  // trace_macros!( false );

}

//

test_suite!
{
  basic,
  canonical_test,
}
