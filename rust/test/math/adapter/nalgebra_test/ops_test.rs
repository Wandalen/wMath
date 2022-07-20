use test_tools::*;
use math_adapter::prelude::*;
use math_adapter::X2;

//

tests_impls!
{
  ///
  /// Tests for X2 conversion function as clone_as_foreign, as_foreign, as_foreign_mut .
  ///

  fn convertion_foreign()
  {
    type T = i8;

    crate::macro_foreign_x2::macro_test_foreign_x2_as_foreign!( nalgebra::Vector2 ; T );

  }

  ///
  /// Operations with dereferencing.
  ///

  fn operation_deref()
  {
    type T = i8;

    crate::macro_foreign_x2::macro_test_foreign_x2_operation_deref!( nalgebra::Vector2 ; T );

  }

  ///
  /// Interoperability with cgmath.
  ///

  fn inter_cgmath()
  {
    #[ cfg( feature = "cgmath" ) ]
    {

      type T = i8;

      /* test.case = "using add operator of nalgebra"; */
      {
        let src1 = cgmath::Vector2::< T >::make( 1, 2 );
        let src2 = nalgebra::Vector2::< T >::make( 3, 4 );
        let got = src1.as_nalgebra() + src2;
        let exp = nalgebra::Vector2::< T >::make( 4, 6 );
        a_id!( got, exp );
      }

      /* test.case = "using either `cgmath's` and `nalgebra's` implementation of sum"; */
      {
        // use cgmath::Array;
        let src = X2::< T >::make( 1, 2 );
        let got = src.as_cgmath().sum();
        a_id!( got, 3 );
        let got = src.as_nalgebra().sum();
        a_id!( got, 3 );
      }

    }
  }
}

//

tests_index!
{
  convertion_foreign,
  operation_deref,
  inter_cgmath,
}
