#![ allow( unused_imports ) ]

use wtest_basic::*;
use core::mem::size_of;
use wmath::prelude::*;
use num_traits::cast::cast;
use crate::tools::*;
use crate::x2_with_records_test_for;

///
/// One test should be ordinary to exclude possibility of problems with macro.
///

fn _basic()
{
  type T = i8;

  /* test.case = "size"; */
  {
    assert_eq!( size_of::< winit::dpi::PhysicalSize::< T > >(), size_of::< ( T, T ) >() );
    assert_eq!( size_of::< winit::dpi::PhysicalSize::< T > >(), size_of::< [ T ; 2 ] >() );
    assert_eq!( size_of::< winit::dpi::PhysicalSize::< T > >(), 2 );
  }

  /* test.case = "value of elements"; */
  {
    let got = winit::dpi::PhysicalSize::< i8 >{ width : 1, height : 2 };
    assert_eq!( got.width, 1 );
    assert_eq!( got.height, 2 );
    assert_eq!( got._0(), 1 );
    assert_eq!( got._1(), 2 );
  }

  /* test.case = "operator add"; */
  {
    let src1 = winit::dpi::PhysicalSize::< i8 >{ width : 1, height : 2 };
    let src2 = winit::dpi::PhysicalSize::< i8 >{ width : 2, height : 3 };
    let got = src1.as_canonical() + src2.as_canonical();
    let exp = wmath::x2::< i8 >( 3, 5 );
    assert_eq!( got, exp );
  }

}

//

fn _canonical_test()
{
  // x2_with_records_test_for!( winit::dpi::PhysicalSize, width, height ; i8, i16, i32, i64, i128 );
  // x2_with_records_test_for!( winit::dpi::PhysicalSize, width, height ; u8, u16, u32, u64, u128 );
  // x2_with_records_test_for!( winit::dpi::PhysicalSize, width, height ; f32, f64 );
  // x2_with_records_test_for!( winit::dpi::PhysicalSize, width, height ; f32 );
}

//

test_suite!
{
  basic,
  canonical_test,
}
