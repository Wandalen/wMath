#![ allow( unused_imports ) ]

use test_tools::*;
use core::mem::size_of;
use num_traits::cast::cast;
use math_adapter::prelude::*;
use math_adapter::X2;
use crate::{ num };

//

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
      a_id!( size_of::< cgmath::Vector2::< T > >(), size_of::< ( T, T ) >() );
      a_id!( size_of::< cgmath::Vector2::< T > >(), size_of::< [ T ; 2 ] >() );
      a_id!( size_of::< cgmath::Vector2::< T > >(), 2 );
    }

    /* test.case = "value of elements"; */
    {
      let got = cgmath::Vector2::< T >{ x : 1, y : 2 };
      a_id!( got.x, 1 );
      a_id!( got.y, 2 );
      a_id!( got._0(), 1 );
      a_id!( got._1(), 2 );
    }

    /* making a new using the module */
    {
      let got = math_adapter::cgmath::Vector2::< T >::make( 1, 2 );
      let exp = cgmath::Vector2::< T >{ x : 1, y : 2 };
      a_id!( got, exp );
    }

  }

  ///
  /// Parametrized test.
  ///

  fn canonical()
  {

    math_adapter::for_each!
    (
      crate::macro_foreign_x2::macro_test_foreign_x2_number_for_each,
      { cgmath::Vector2, x, y },
      { cgmath::Point2, x, y },
    );

    // trace_macros!( true );
    // macro_test_foreign_x2_number!( ( cgmath::Point2, x, y ) f32 );
    // trace_macros!( false );
  }

  ///
  /// Tests for X2 conversion function. Names are implementation-specific. .
  ///

  fn convertion_as_specific()
  {
    type T = i8;

    crate::macro_foreign_x2::macro_test_foreign_x2_as_specific!( cgmath::Vector2, cgmath ; T );
  }
}

//

tests_index!
{
  basic,
  canonical,
  convertion_as_specific,
}
