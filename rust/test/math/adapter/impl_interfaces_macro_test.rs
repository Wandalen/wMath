use test_tools::*;
use math_adapter::X2;
use super::*;

tests_impls!
{
  fn impls_for_structure()
  {
    let x2 = X2( 1, 2 );
    assert!( implements!( x2 => X2NominalInterface ) );
    assert!( implements!( x2 => X2Interface ) );
    assert!( implements!( x2 => X2BasicInterface ) );
    assert!( implements!( x2 => X2CanonicalInterface ) );
  }

  //

  fn impls_for_array()
  {
    let array = [ 1, 2 ];
    assert!( implements!( array => X2NominalInterface ) );
    assert!( implements!( array => X2Interface ) );
    assert!( implements!( array => X2BasicInterface ) );
    assert!( implements!( array => X2CanonicalInterface ) );
  }

  //

  fn impls_for_slice()
  {
    let slice = &[ 1, 2 ][..];
    assert!( implements!( slice => X2NominalInterface ) );
    assert!( implements!( slice => X2Interface ) );
  }

  //

  fn impls_for_tuple()
  {
    let tuple = ( 1, 2 );
    assert!( implements!( tuple => X2NominalInterface ) );
    assert!( implements!( tuple => X2Interface ) );
    assert!( implements!( tuple => X2BasicInterface ) );
    assert!( implements!( tuple => X2CanonicalInterface ) );
  }
}

//

tests_index!
{
  impls_for_structure,
  impls_for_array,
  impls_for_slice,
  impls_for_tuple,
}
