#![ allow( unused_imports ) ]

use wmath::prelude::*;
use wmath::X2;

fn main()
{
  let x2_original = X2::< u8 >( 1, 3 );
  println!( "{:?}", x2_original );
  /* log : X2(1, 3) */
  let x2_to_array = x2_original.clone_as_array();
  println!( "{:?}", x2_to_array );
  /* log : [1, 3] */
}
