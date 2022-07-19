// #![ allow( unused_imports ) ]

use test_tools::*;
use core::mem::size_of;
use math_adapter::prelude::*;
use math_adapter::X2;
use crate::{ num };

tests_impls!
{

  ///
  /// One test should be ordinary to exclude possibility of problems with macro.
  ///

  fn basic()
  {
    type T = i8;

    /* test.case = "size"; */
    {
      a_id!( size_of::< winit::dpi::PhysicalSize::< T > >(), size_of::< ( T, T ) >() );
      a_id!( size_of::< winit::dpi::PhysicalSize::< T > >(), size_of::< [ T ; 2 ] >() );
      a_id!( size_of::< winit::dpi::PhysicalSize::< T > >(), 2 );
    }

    /* test.case = "value of elements"; */
    {
      let got = winit::dpi::PhysicalSize::< i8 >{ width : 1, height : 2 };
      a_id!( got.width, 1 );
      a_id!( got.height, 2 );
      a_id!( got._0(), 1 );
      a_id!( got._1(), 2 );
    }

    // /* test.case = "operator add"; */
    // {
    //   let src1 = winit::dpi::PhysicalSize::< i8 >{ width : 1, height : 2 };
    //   let src2 = winit::dpi::PhysicalSize::< i8 >{ width : 2, height : 3 };
    //   let got = src1.as_canonical() + src2.as_canonical();
    //   let exp = math_adapter::X2::< i8 >( 3, 5 );
    //   a_id!( got, exp );
    // }

  }

  //

  fn physical_size()
  {

    math_adapter::for_each!
    (
      crate::macro_foreign_x2::macro_test_foreign_x2_number_for_each,
      { winit::dpi::PhysicalSize, width, height },
      { winit::dpi::LogicalSize, width, height },
      { winit::dpi::PhysicalPosition, x, y },
      { winit::dpi::LogicalPosition, x, y },
    );

    // trace_macros!( true );
    // macro_test_foreign_x2_number!( ( winit::dpi::PhysicalSize, width, height ) f32 );
    // trace_macros!( false );

  }

  ///
  /// Basic test of interoperability of `winit` with `cgmath`.
  ///

  #[ cfg( all( feature = "winit", feature = "cgmath" ) ) ]
  fn cgmath_winit_interoperability()
  {

    /* test.case = "use cgmath vectors for operations on winit vectors"; */
    {
      let src1 = winit::dpi::PhysicalSize::< i8 >::make( 3, 2 );
      let src2 = winit::dpi::PhysicalSize::< i8 >::make( 0, 1 );
      let got = src1.as_cgmath() - src2.as_cgmath();
      let exp = cgmath::Vector2::< i8 >::make( 3, 1 );
      a_id!( got, exp );
    }

  }

  ///
  /// Interoperability with winit.
  ///

  #[ cfg( all( feature = "winit" ) ) ]
  #[ cfg( nalgebra_ops ) ]
  fn inter_winit()
  {
    type T = i8;

    /* test.case = "basic"; */
    {
      let src1 = winit::dpi::PhysicalSize::< T >::make( 1, 3 );
      let got = src1.as_canonical().sum();
      let exp = 4;
      a_id!( got, exp );
    }

  }

}

//

tests_index!
{
  basic,
  physical_size,
  cgmath_winit_interoperability,
  inter_winit,
}

/* zzz : teach test_suite to understand directives */
