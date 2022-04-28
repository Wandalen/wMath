/* qqq : implement please */ /* aaa : Dmytro : done */

use wmath::*;

fn main()
{
  let vec2 = ix2_cgmath::new( 1, 3 );
  println!( "{:?}", vec2 );
  /* log : Vector2 [1, 3] */
  let vec3 = vec2.extend( 2 );
  println!( "{:?}", vec3 );
  /* log : Vector3 [1, 3, 2] */
}
