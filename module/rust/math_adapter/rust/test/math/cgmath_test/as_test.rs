#![ allow( unused_imports ) ]

use wtest_basic::*;
use crate::test_tools::*;
use math_adapter::prelude::*;
// use math_adapter::{ X2NominalInterface, X2BasicInterface, X2CanonicalInterface };
use math_adapter::X2;
use cgmath::Vector2 as X2Native;

/* xxx : is assign test implemented for natives? */

///
/// Tests for X2 conversion function as clone_as_native, as_native, as_native_mut .
///

fn _convertion_native()
{
  #[ allow( dead_code ) ]
  type T = i8;

  #[ cfg( feature = "cgmath" ) ]
  #[
    cfg( all
    (
      not( feature = "nalgebra_ops" ),
      not( all( feature = "default_ops", feature = "nalgebra" ) ),
      any( feature = "default_ops", feature = "cgmath_ops" ),
    ))
  ]
  crate::macro_test_x2_as_native!( X2Native ; T );

}

///
/// Tests for X2 conversion function. Names are implementation-specific. .
///

#[ test ]
fn _convertion_as_specific()
{
  type T = i8;

  // xxx : make macro. use paste
  // paste!(
  //     Struct1::[<prefix_ $postfix>]()
  // );

  /* test.case = "clone_as_cgmath"; */
  {
    let src = X2::< T >::make( 1, 2 );
    let got = src.clone_as_cgmath();
    let exp = X2Native::< T >::make( 1, 2 );
    assert_eq!( got, exp );
    assert!( !mem_same_ptr( &got, &src ) );
  }

  /* test.case = "as_cgmath"; */
  {
    let src = X2::< T >::make( 1, 2 );
    let got = src.as_cgmath();
    let exp = X2Native::< T >::make( 1, 2 );
    assert_eq!( *got, exp );
    assert!( mem_same_region( got, &src ) );
  }

  /* test.case = "as_cgmath_mut"; */
  {
    let mut src = X2::< T >::make( 1, 2 );
    let got = src.as_cgmath_mut();
    let exp = X2Native::< T >::make( 1, 2 );
    assert_eq!( *got, exp );
    got.assign( ( 11, 22 ) );
    let exp = X2::< T >::make( 11, 22  );
    assert_eq!( src, exp );
  }

}

///
/// Operations with dereferencing.
///

#[
  cfg( all
  (
    not( feature = "nalgebra_ops" ),
    not( all( feature = "default_ops", feature = "nalgebra" ) ),
    any( feature = "default_ops", feature = "cgmath_ops" ),
  ))
]
#[ test ]
fn operation_deref()
{
  type T = i8;
  // use math_adapter::cgmath::as_native::*; xxx
  use cgmath::Array;

  crate::macro_test_x2_operation_deref!( X2Native ; T );

}

//

#[ cfg( all( feature = "winit", feature = "cgmath" ) ) ]
#[ test ]
fn cgmath_winit_interoperability_test()
{

  /* test.case = "use cgmath vectors for operations on winit vectors"; */
  {
    let src1 = winit::dpi::PhysicalSize::< i8 >::make( 3, 2 );
    let src2 = winit::dpi::PhysicalSize::< i8 >::make( 0, 1 );
    let got = src1.as_cgmath() - src2.as_cgmath();
    let exp = X2Native::< i8 >::make( 3, 1 );
    assert_eq!( got, exp );
  }

}

//

test_suite!
{
  convertion_native,
}
