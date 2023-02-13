use test_tools::*;
use math_adapter::
{
  X2,
  X3,
  X4,
};
use super::*;

tests_impls!
{
  fn impls_x2_for_structure()
  {
    let x2 = X2( 1, 2 );
    assert!( implements!( x2 => X2NominalInterface ) );
    assert!( implements!( x2 => X2Interface ) );
    assert!( implements!( x2 => X2BasicInterface ) );
    assert!( implements!( x2 => X2CanonicalInterface ) );
  }

  //

  fn impls_x2_for_array()
  {
    let array = [ 1, 2 ];
    assert!( implements!( array => X2NominalInterface ) );
    assert!( implements!( array => X2Interface ) );
    assert!( implements!( array => X2BasicInterface ) );
    assert!( implements!( array => X2CanonicalInterface ) );
  }

  //

  fn impls_x2_for_slice()
  {
    let slice = &[ 1, 2 ][..];
    assert!( implements!( slice => X2NominalInterface ) );
    assert!( implements!( slice => X2Interface ) );
  }

  //

  fn impls_x2_for_tuple()
  {
    let tuple = ( 1, 2 );
    assert!( implements!( tuple => X2NominalInterface ) );
    assert!( implements!( tuple => X2Interface ) );
    assert!( implements!( tuple => X2BasicInterface ) );
    assert!( implements!( tuple => X2CanonicalInterface ) );
  }

  //

    fn impls_x3_for_structure()
  {
    let x3 = X3( 1, 2, 3 );
    assert!( implements!( x3 => X3NominalInterface ) );
    assert!( implements!( x3 => X3Interface ) );
    assert!( implements!( x3 => X3BasicInterface ) );
    assert!( implements!( x3 => X3CanonicalInterface ) );
  }

  //

  fn impls_x3_for_array()
  {
    let array = [ 1, 2, 3 ];
    assert!( implements!( array => X3NominalInterface ) );
    assert!( implements!( array => X3Interface ) );
    assert!( implements!( array => X3BasicInterface ) );
    assert!( implements!( array => X3CanonicalInterface ) );
  }

  //

  fn impls_x3_for_slice()
  {
    let slice = &[ 1, 2, 3 ][..];
    assert!( implements!( slice => X3NominalInterface ) );
    assert!( implements!( slice => X3Interface ) );
  }

  //

  fn impls_x3_for_tuple()
  {
    let tuple = ( 1, 2, 3 );
    assert!( implements!( tuple => X3NominalInterface ) );
    assert!( implements!( tuple => X3Interface ) );
    assert!( implements!( tuple => X3BasicInterface ) );
    assert!( implements!( tuple => X3CanonicalInterface ) );
  }

  //

    fn impls_x4_for_structure()
  {
    let x4 = X4( 1, 2, 3, 4 );
    assert!( implements!( x4 => X4NominalInterface ) );
    assert!( implements!( x4 => X4Interface ) );
    assert!( implements!( x4 => X4BasicInterface ) );
    assert!( implements!( x4 => X4CanonicalInterface ) );
  }

  //

  fn impls_x4_for_array()
  {
    let array = [ 1, 2, 3, 4 ];
    assert!( implements!( array => X4NominalInterface ) );
    assert!( implements!( array => X4Interface ) );
    assert!( implements!( array => X4BasicInterface ) );
    assert!( implements!( array => X4CanonicalInterface ) );
  }

  //

  fn impls_x4_for_slice()
  {
    let slice = &[ 1, 2, 3, 4 ][..];
    assert!( implements!( slice => X4NominalInterface ) );
    assert!( implements!( slice => X4Interface ) );
  }

  //

  fn impls_x4_for_tuple()
  {
    let tuple = ( 1, 2, 3, 4 );
    assert!( implements!( tuple => X4NominalInterface ) );
    assert!( implements!( tuple => X4Interface ) );
    assert!( implements!( tuple => X4BasicInterface ) );
    assert!( implements!( tuple => X4CanonicalInterface ) );
  }
}

//

tests_index!
{
  impls_x2_for_structure,
  impls_x2_for_array,
  impls_x2_for_slice,
  impls_x2_for_tuple,
  impls_x3_for_structure,
  impls_x3_for_array,
  impls_x3_for_slice,
  impls_x3_for_tuple,
  impls_x4_for_structure,
  impls_x4_for_array,
  impls_x4_for_slice,
  impls_x4_for_tuple,
}
