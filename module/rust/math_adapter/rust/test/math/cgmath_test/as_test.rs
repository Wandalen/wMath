#![ allow( unused_imports ) ]

use wtest_basic::*;
use crate::test_tools::*;
use math_adapter::prelude::*;
use math_adapter::X2;
use cgmath::Vector2 as X2Foreign;

///
/// Tests for X2 conversion function as clone_as_foreign, as_foreign, as_foreign_mut .
///

fn _convertion_foreign()
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
  crate::macro_test_x2_as_foreign!( X2Foreign ; T );

}

///
/// Tests for X2 conversion function. Names are implementation-specific. .
///

#[ test ]
fn _convertion_as_specific()
{
  type T = i8;

  crate::macro_test_x2_as_specific!( cgmath::Vector2, cgmath ; T );

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
  crate::macro_test_x2_operation_deref!( X2Foreign ; T );
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
    let exp = X2Foreign::< i8 >::make( 3, 1 );
    assert_eq!( got, exp );
  }

}

//

test_suite!
{
  convertion_foreign,
}
