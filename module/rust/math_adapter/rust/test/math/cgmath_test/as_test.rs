#![ allow( unused_imports ) ]

use wtest_basic::*;
use crate::test_tools::*;
// use math_adapter::prelude::*;
use math_adapter::{ X2NominalInterface, X2BasicInterface, X2CanonicalInterface };
use math_adapter::X2;

/* xxx : is assign test implemented for natives? */

///
/// Convertion or reinterpretation from one type to another.
///

fn _convertion()
{
  type T = i8;
  use math_adapter::cgmath::as_native::*;

  /* test.case = "clone_as_cgmath"; */
  {
    let src = X2::< T >::make( 1, 2 );
    let got = src.clone_as_cgmath();
    let exp = cgmath::Vector2::< T >::make( 1, 2 );
    assert_eq!( got, exp );
    assert!( !mem_same_ptr( &got, &src ) );
    let got = src.clone_as_native();
    let exp = cgmath::Vector2::< T >::make( 1, 2 );
    assert_eq!( got, exp );
    assert!( !mem_same_ptr( &got, &src ) );
  }

  /* test.case = "as_cgmath"; */
  {
    let src = X2::< T >::make( 1, 2 );
    let got = src.as_cgmath();
    let exp = cgmath::Vector2::< T >::make( 1, 2 );
    assert_eq!( *got, exp );
    assert!( mem_same_region( got, &src ) );
    let got = src.as_native();
    let exp = cgmath::Vector2::< T >::make( 1, 2 );
    assert_eq!( *got, exp );
    assert!( mem_same_region( got, &src ) );
  }

  /* test.case = "as_cgmath_mut"; */
  {
    let mut src = X2::< T >::make( 1, 2 );
    let got = src.as_cgmath_mut();
    let exp = cgmath::Vector2::< T >::make( 1, 2 );
    assert_eq!( *got, exp );
    got.assign( ( 11, 22 ) );
    let exp = X2::< T >::make( 11, 22  );
    assert_eq!( src, exp );
    let mut src = X2::< T >::make( 1, 2 );
    let got = src.as_native_mut();
    let exp = cgmath::Vector2::< T >::make( 1, 2 );
    assert_eq!( *got, exp );
    got.assign( ( 11, 22 ) );
    let exp = X2::< T >::make( 11, 22  );
    assert_eq!( src, exp );
  }

}

///
/// Operations without dereferencing.
///

#[ cfg( feature = "cgmath_ops" ) ]
#[ test ]
fn operation()
{
  type T = i8;
  use math_adapter::cgmath::as_native::*;

  /* test.case = "neg"; */
  {
    let src1 = X2::< T >::make( 4, 3 );
    let got = - src1;
    let exp = X2::< T >::make( -4, -3 );
    assert_eq!( got, exp );
  }

  /* test.case = "add"; */
  {
    let src1 = X2::< T >::make( 4, 3 );
    let src2 = X2::< T >::make( 2, 1 );
    let got = src1 + src2;
    let exp = X2::< T >::make( 6, 4 );
    assert_eq!( got, exp );
  }

  /* test.case = "sub"; */
  {
    let src1 = X2::< T >::make( 4, 3 );
    let src2 = X2::< T >::make( 1, 2 );
    let got = src1 - src2;
    let exp = X2::< T >::make( 3, 1 );
    assert_eq!( got, exp );
  }

}

///
/// Operations with dereferencing.
///

#[
  cfg( all
  (
    not( all( feature = "default_ops", feature = "nalgebra" ) ),
    any( feature = "default_ops", feature = "cgmath" ),
  ))
]
#[ test ]
fn operation_deref()
{
  type T = i8;
  use math_adapter::cgmath::as_native::*;

  /* test.case = "neg"; */
  {
    let src1 = X2::< T >::make( 4, 3 );
    let got = - *src1;
    let exp = math_adapter::cgmath::X2::< T >::make( -4, -3 );
    assert_eq!( got, exp );
  }

  /* test.case = "add"; */
  {
    let src1 = X2::< T >::make( 4, 3 );
    let src2 = X2::< T >::make( 2, 1 );
    let got = *src1 + *src2;
    let exp = math_adapter::cgmath::X2::< T >::make( 6, 4 );
    assert_eq!( got, exp );
  }

  /* test.case = "sub"; */
  {
    let src1 = X2::< T >::make( 4, 3 );
    let src2 = X2::< T >::make( 1, 2 );
    let got = *src1 - *src2;
    let exp = math_adapter::cgmath::X2::< T >::make( 3, 1 );
    assert_eq!( got, exp );
  }

  /* test.case = "dereferenced method"; */
  {
    let src1 = X2::< T >::make( 4, 3 );
    let src2 = X2::< T >::make( 1, 2 );
    let got = src1.perp_dot( *src2 );
    let exp = 5;
    assert_eq!( got, exp );
  }

}

//

#[
  cfg( all
  (
    not( all( feature = "default_ops", feature = "nalgebra" ) ),
    any( feature = "default_ops", feature = "cgmath" ),
  ))
]
#[ cfg( all( feature = "winit" ) ) ]
#[ test ]
fn cgmath_winit_interoperability_test()
{

  /* test.case = "use cgmath vectors for operations on winit vectors"; */
  {
    // use AsCgmathInterface;
    let src1 = winit::dpi::PhysicalSize::< i8 >::make( 3, 2 );
    let src2 = winit::dpi::PhysicalSize::< i8 >::make( 0, 1 );
    let got = src1.as_cgmath() - src2.as_cgmath();
    let exp = cgmath::Vector2::< i8 >::make( 3, 1 );
    assert_eq!( got, exp );
  }

  // /* test.case = "use cgmath vectors for operations on winit vectors dereferencing"; */
  // {
  //   // use AsCgmathInterface;
  //   let src1 = winit::dpi::PhysicalSize::< i8 >::make( 3, 2 );
  //   let src2 = winit::dpi::PhysicalSize::< i8 >::make( 0, 1 );
  //   let got = *src1 - *src2;
  //   let exp = cgmath::Vector2::< i8 >::make( 3, 1 );
  //   assert_eq!( got, exp );
  // }

}

//

test_suite!
{
  convertion,
}
