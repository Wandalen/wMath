#![ allow( unused_imports ) ]

use test_tools::*;
use core::mem::size_of;
use num_traits::cast::cast;
use math_adapter::prelude::*;
use math_adapter::X3;
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
      a_id!( size_of::< cgmath::Vector3::< T > >(), size_of::< ( T, T, T ) >() );
      a_id!( size_of::< cgmath::Vector3::< T > >(), size_of::< [ T ; 3 ] >() );
      a_id!( size_of::< cgmath::Vector3::< T > >(), 3 );
    }

    /* test.case = "value of elements"; */
    {
      let got = cgmath::Vector3::< T >{ x : 1, y : 2, z : 3 };
      a_id!( got.x, 1 );
      a_id!( got.y, 2 );
      a_id!( got.z, 3 );
      a_id!( got._0(), 1 );
      a_id!( got._1(), 2 );
      a_id!( got._2(), 3 );
    }

    /* making a new using the module */
    {
      let got = math_adapter::cgmath::Vector3::< T >::make( 1, 2, 3 );
      let exp = cgmath::Vector3::< T >{ x : 1, y : 2, z : 3 };
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
      crate::macro_foreign_x3::macro_test_foreign_x3_number_for_each,
      { cgmath::Vector3, x, y, z },
      { cgmath::Point3, x, y, z },
    );
  }

  ///
  /// Tests for X3 conversion function. Names are implementation-specific. .
  ///

  fn convertion_as_specific()
  {
    type T = i8;

    crate::macro_foreign_x3::macro_test_foreign_x3_as_specific!( cgmath::Vector3, cgmath ; T );
  }
}

//

tests_index!
{
  basic,
  canonical,
  convertion_as_specific,
}
