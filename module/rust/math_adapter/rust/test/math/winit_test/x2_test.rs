// #![ allow( unused_imports ) ]

use wtest_basic::*;
use core::mem::size_of;
use math_adapter::prelude::*;
use math_adapter::X2;
use crate::test_tools::*;
use crate::{ num };

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

  // /* test.case = "operator add"; */
  // {
  //   let src1 = winit::dpi::PhysicalSize::< i8 >{ width : 1, height : 2 };
  //   let src2 = winit::dpi::PhysicalSize::< i8 >{ width : 2, height : 3 };
  //   let got = src1.as_canonical() + src2.as_canonical();
  //   let exp = math_adapter::X2::< i8 >( 3, 5 );
  //   assert_eq!( got, exp );
  // }

}

//

fn _physical_size_test()
{

  crate::macro_x2::macro_test_x2_with_records!( winit::dpi::PhysicalSize, width, height ; i8, i16, i32, i64, i128 );
  crate::macro_x2::macro_test_x2_with_records!( winit::dpi::PhysicalSize, width, height ; u8, u16, u32, u64, u128 );
  crate::macro_x2::macro_test_x2_with_records!( winit::dpi::PhysicalSize, width, height ; f32, f64 );

  crate::macro_x2::macro_test_x2_with_records!( winit::dpi::LogicalSize, width, height ; i8, i16, i32, i64, i128 );
  crate::macro_x2::macro_test_x2_with_records!( winit::dpi::LogicalSize, width, height ; u8, u16, u32, u64, u128 );
  crate::macro_x2::macro_test_x2_with_records!( winit::dpi::LogicalSize, width, height ; f32, f64 );

  crate::macro_x2::macro_test_x2_with_records!( winit::dpi::PhysicalPosition, x, y ; i8, i16, i32, i64, i128 );
  crate::macro_x2::macro_test_x2_with_records!( winit::dpi::PhysicalPosition, x, y ; u8, u16, u32, u64, u128 );
  crate::macro_x2::macro_test_x2_with_records!( winit::dpi::PhysicalPosition, x, y ; f32, f64 );

  crate::macro_x2::macro_test_x2_with_records!( winit::dpi::LogicalPosition, x, y ; i8, i16, i32, i64, i128 );
  crate::macro_x2::macro_test_x2_with_records!( winit::dpi::LogicalPosition, x, y ; u8, u16, u32, u64, u128 );
  crate::macro_x2::macro_test_x2_with_records!( winit::dpi::LogicalPosition, x, y ; f32, f64 );

}

///
/// Basic test of interoperability of `winit` with `cgmath`.
///

#[ cfg( all( feature = "winit", feature = "cgmath" ) ) ]
#[ test ]
fn cgmath_winit_interoperability_test()
{

  /* test.case = "use cgmath vectors for operations on winit vectors"; */
  {
    let src1 = winit::dpi::PhysicalSize::< i8 >::make( 3, 2 );
    let src2 = winit::dpi::PhysicalSize::< i8 >::make( 0, 1 );
    let got = src1.as_cgmath() - src2.as_cgmath();
    let exp = cgmath::Vector2::< i8 >::make( 3, 1 );
    assert_eq!( got, exp );
  }

}

///
/// Interoperability with winit.
///

#[ cfg( all( feature = "winit" ) ) ]
#[ cfg( any( feature = "nalgebra_ops", feature = "default_ops" ) ) ]
#[ test ]
fn inter_winit()
{
  type T = i8;

  /* test.case = "basic"; */
  {
    let src1 = winit::dpi::PhysicalSize::< T >::make( 1, 3 );
    let got = src1.as_canonical().sum();
    let exp = 4;
    assert_eq!( got, exp );
  }

}

//

test_suite!
{
  basic,
  physical_size_test,
}
/* xxx : extend the macro */
