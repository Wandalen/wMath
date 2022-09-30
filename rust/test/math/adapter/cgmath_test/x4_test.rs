#![ allow( unused_imports ) ]

use test_tools::*;
use core::mem::size_of;
use num_traits::cast::cast;
use math_adapter::prelude::*;
use math_adapter::X4;
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
      a_id!( size_of::< cgmath::Vector4::< T > >(), size_of::< ( T, T, T, T ) >() );
      a_id!( size_of::< cgmath::Vector4::< T > >(), size_of::< [ T ; 4 ] >() );
      a_id!( size_of::< cgmath::Vector4::< T > >(), 4 );
    }

    /* test.case = "value of elements"; */
    {
      let got = cgmath::Vector4::< T >{ x : 1, y : 2, z : 3, w : 4 };
      a_id!( got.x, 1 );
      a_id!( got.y, 2 );
      a_id!( got.z, 3 );
      a_id!( got.w, 4 );
      a_id!( got._0(), 1 );
      a_id!( got._1(), 2 );
      a_id!( got._2(), 3 );
      a_id!( got._3(), 4 );
    }

    /* making a new using the module */
    {
      let got = math_adapter::cgmath::Vector4::< T >::make( 1, 2, 3, 4 );
      let exp = cgmath::Vector4::< T >{ x : 1, y : 2, z : 3, w : 4 };
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
      crate::macro_foreign_x4::macro_test_foreign_x4_number_for_each,
      { cgmath::Vector4, x, y, z, w },
      // cgmath doesn't have the Point4 type.
      // { cgmath::Point4, x, y, z, w },
    );
  }

  ///
  /// Tests for X4 conversion function. Names are implementation-specific. .
  ///

  fn convertion_as_specific()
  {
    type T = i8;

    crate::macro_foreign_x4::macro_test_foreign_x4_as_specific!( cgmath::Vector4, cgmath ; T );
  }
}

//

tests_index!
{
  basic,
  canonical,
  convertion_as_specific,
}
