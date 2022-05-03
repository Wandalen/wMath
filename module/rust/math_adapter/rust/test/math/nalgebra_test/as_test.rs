#![ allow( unused_imports ) ]

use wtest_basic::*;
use crate::test_tools::*;
use math_adapter::prelude::*;
use math_adapter::X2;
use nalgebra::Vector2 as X2Foreign;

///
/// Tests for X2 conversion function as clone_as_foreign, as_foreign, as_foreign_mut .
///

fn _convertion_foreign()
{
  #[ allow( dead_code ) ]
  type T = i8;

  #[ cfg( any( feature = "nalgebra_ops", feature = "default_ops" ) ) ]
  crate::macro_test_x2_as_foreign!( X2Foreign ; T );

}

///
/// Tests for X2 conversion function. Names are implementation-specific. .
///

#[ test ]
fn _convertion_as_specific()
{
  type T = i8;

  crate::macro_test_x2_as_specific!( X2Foreign, nalgebra ; T );

}

///
/// Operations with dereferencing.
///

#[ cfg( any( feature = "nalgebra_ops", feature = "default_ops" ) ) ]
#[ test ]
fn operation_deref()
{
  type T = i8;

  crate::macro_test_x2_operation_deref!( X2Foreign ; T );

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

///
/// Interoperability with cgmath.
///

#[ cfg( all( feature = "cgmath" ) ) ]
// #[ cfg( any( feature = "nalgebra_ops", feature = "default_ops" ) ) ]
#[ test ]
fn inter_cgmath()
{
  type T = i8;

  /* test.case = "using add operator of nalgebra"; */
  {
    let src1 = cgmath::Vector2::< T >::make( 1, 2 );
    let src2 = X2Foreign::< T >::make( 3, 4 );
    let got = src1.as_nalgebra() + src2;
    let exp = X2Foreign::< T >::make( 4, 6 );
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

//

test_suite!
{
  convertion_foreign,
}

/* zzz : teach test_suite to understand directives */
/* xxx : in cgmath implement interfaces for Point2 */
