use math_adapter::prelude::*;

fn main()
{
  let src1 = math_adapter::X2::make( 1, 3 );
  assert_eq!( src1.x(), 3 );
  assert_eq!( src1.y(), 3 );
}
