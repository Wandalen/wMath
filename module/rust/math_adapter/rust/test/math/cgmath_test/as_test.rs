#![ allow( unused_imports ) ]

use wtest_basic::*;
use crate::tools::*;
use math_adapter::prelude::*;
use math_adapter::X2;

///
/// Convertion or reinterpretation from one type to another.
///

fn _convertion()
{
  type T = i8;

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

#[ cfg( feature = "cgmath_ops" ) ]
#[ test ]
fn operation_deref()
{
  type T = i8;

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

test_suite!
{
  convertion,
}
