#![ allow( unused_imports ) ]

//!
//! Feature `*_ops` means to request to use operators and function of math lib of choice. But instead of choosing a single back-end math lib, you may use several. Use methods `as_*_clone()`, `as_*`, `as_*_mut` to either convert or reinterpret original math object into analog of such of chosen back-end. You don't have to use the same back-end in every call, you may choose which math lib for a specific call and combine the best of each math lib.
//!

use math_adapter::prelude::*;
use math_adapter::X2;

fn main()
{

  // ! compile time error, because if no `*_ops` feature was chosen
  // {
  //   let src = X2::make( 1, 3 ); /* make a canonical 2D vector */
  //   println!( "src.sum() : {:?}", src.sum() ); /* use `sum()` of chosen math lib back-end */
  // }

  // enable feature *_ops should be enabled to get access to functions
  #[ cfg( all( feature = "cgmath", feature = "nalgebra" ) ) ]
  {
    let src = X2::make( 1, 3 ); /* make a canonical 2D vector */
    println!( "src.as_cgmath().sum() : {:?}", src.as_cgmath().sum() ); /* use `sum()` of `cgmath` */
    println!( "src.as_nalgebra().sum() : {:?}", src.as_nalgebra().sum() ); /* use `sum()` of `nalgebra` */
  }

  // you can convert / reinterpret any vector.
  // for example you can create `cgmath::Vector2`, but apply a function of `nalgebra::Vector2`
  #[ cfg( all( feature = "cgmath", feature = "nalgebra" ) ) ]
  {
    let src = math_adapter::cgmath::Vector2::< i32 >::make( 1, 3 ); /* make a `cgmath` 2D vector */
    println!( "src.as_nalgebra().sum() : {:?}", src.as_nalgebra().sum() ); /* use `sum()` of `nalgebra` */
  }

}
