/* qqq : implement please */ /* aaa : Dmytro : done */

use wmath::*;

fn main()
{
  let x2_original = x2::< u8 >( 1, 3 );
  println!( "{:?}", x2_original );
  /* log : x2(1, 3) */
  let x2_to_array = x2_original.clone_as_array();
  println!( "{:?}", x2_to_array );
  /* log : [1, 3] */
}
