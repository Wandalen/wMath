
//!
//! Select a feature `*_ops` to reuse operators and function of math lib of choice.
//!

fn main()
{
  #[ cfg( feature = "use_std" ) ]
  {
    #[ allow( unused_imports ) ]
    use math_adapter::prelude::*;
    #[ allow( unused_imports ) ]
    use math_adapter::X2;

    // if back-end math lib is chosen then operators and functions are available
    #[ cfg( feature = "cgmath_ops" ) ]
    {
      let src1 = X2::make( 1, 2 );
      let src2 = X2::make( 3, 4 );
      let got = src1 + src2;
      let exp = X2::make( 4, 6 );
      assert_eq!( got, exp );
      println!( "src1 + src2 : {:?}", got );
    }

    // enable feature *_ops to get access to functions
    #[ cfg( feature = "cgmath_ops" ) ]
    {
      let src = X2::make( 1, 2 );
      assert_eq!( src.sum(), 3 ); /* sum comes from `cgmath` */
      println!( "src.sum() : {:?}", src.sum() );
    }
  }
}