#![ allow( unused_imports ) ]

//!
//! Number of elements of a vector is coded in name of type, `X2` for vector of length `2`, `X3` for vector of length 3 and so on. Each structure implements constructors `make()`, `make_nan()`, and `make_default()` to construct a new instance of the type. To get access to elements use either methods `x()`, `y()`, `z()` or `_0()`, `_1()`, `_2()`.
//!

fn main()
{
  #[ cfg( feature = "use_std" ) ]
  {
    use math_adapter::prelude::*;
    use math_adapter::X2;

    // vector of length 2 and its elements
    let src1 = X2::make( 1, 3 );
    assert_eq!( src1.x(), 1 );
    assert_eq!( src1.y(), 3 );
    assert_eq!( src1._0(), 1 );
    assert_eq!( src1._1(), 3 );
  }
}
