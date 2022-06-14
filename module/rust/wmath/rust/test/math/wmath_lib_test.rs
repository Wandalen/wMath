use test_tools::*;

tests_impls!
{
  fn adapter_basic()
  {
    pub use wmath::adapter::prelude::*;

    // test.case = "elements of a vector";
    {
      let src1 = wmath::X2::make( 1, 3 );
      assert_eq!( src1.x(), 1 );
      assert_eq!( src1.y(), 3 );
    }

    // test.case = "operator add";
    #[ cfg( any( feature = "cgmath_ops", feature = "nalgebra_ops" ) ) ]
    {
      let src1 = wmath::X2::make( 1, 2 );
      let src2 = wmath::X2::make( 3, 4 );
      let got = src1 + src2;
      let exp = wmath::X2::make( 4, 6 );
      assert_eq!( got, exp );
    }
  }
}

//

tests_index!
{
  adapter_basic,
}
