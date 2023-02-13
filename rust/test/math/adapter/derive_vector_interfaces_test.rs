use test_tools::*;
use math_adapter::
{
  X2,
  X3,
  X4,
};
use super::*;

#[ derive( VectorInterfaces ) ]
struct TestX2Tuple( u32, u32 );

#[ derive( VectorInterfaces ) ]
struct TestX2TupleWithGenerics< S : ScalarInterface >( S, S );

#[ derive( VectorInterfaces ) ]
struct TestX2Named{ x : u32, y : u32 }

#[ derive( VectorInterfaces ) ]
struct TestX2NamedWithGenerics< S : ScalarInterface >{ x : S, y : S }

#[ derive( VectorInterfaces ) ]
struct TestX3Tuple( u32, u32, u32 );

#[ derive( VectorInterfaces ) ]
struct TestX3TupleWithGenerics< S : ScalarInterface >( S, S, S );

#[ derive( VectorInterfaces ) ]
struct TestX3Named{ x : u32, y : u32, z : u32 }

#[ derive( VectorInterfaces ) ]
struct TestX3NamedWithGenerics< S : ScalarInterface >{ x : S, y : S, z : S }

#[ derive( VectorInterfaces ) ]
struct TestX4Tuple( u32, u32, u32, u32 );

#[ derive( VectorInterfaces ) ]
struct TestX4TupleWithGenerics< S : ScalarInterface >( S, S, S, S );

#[ derive( VectorInterfaces ) ]
struct TestX4Named{ x : u32, y : u32, z : u32, w : u32 }

#[ derive( VectorInterfaces ) ]
struct TestX4NamedWithGenerics< S : ScalarInterface >{ x : S, y : S, z : S, w : S }

tests_impls!
{
  fn impls_x2_for_tuple()
  {
    let tuple = TestX2Tuple( 1, 2 );
    assert!( implements!( tuple => X2NominalInterface ) );
    assert!( implements!( tuple => X2Interface ) );
    assert!( implements!( tuple => X2BasicInterface ) );
    assert!( implements!( tuple => X2CanonicalInterface ) );
  }

  //

  fn impls_x2_for_tuple_with_generics()
  {
    let tuple = TestX2TupleWithGenerics( 1, 2 );
    assert!( implements!( tuple => X2NominalInterface ) );
    assert!( implements!( tuple => X2Interface ) );
    assert!( implements!( tuple => X2BasicInterface ) );
    assert!( implements!( tuple => X2CanonicalInterface ) );
  }

  //

  fn impls_x2_for_named()
  {
    let named = TestX2Named{ x : 1, y : 2 };
    assert!( implements!( named => X2NominalInterface ) );
    assert!( implements!( named => X2Interface ) );
    assert!( implements!( named => X2BasicInterface ) );
    assert!( implements!( named => X2CanonicalInterface ) );
  }

  //

  fn impls_x2_for_named_with_generics()
  {
    let named = TestX2NamedWithGenerics{ x : 1, y : 2 };
    assert!( implements!( named => X2NominalInterface ) );
    assert!( implements!( named => X2Interface ) );
    assert!( implements!( named => X2BasicInterface ) );
    assert!( implements!( named => X2CanonicalInterface ) );
  }

  //

  fn impls_x3_for_tuple()
  {
    let tuple = TestX3Tuple( 1, 2, 3 );
    assert!( implements!( tuple => X3NominalInterface ) );
    assert!( implements!( tuple => X3Interface ) );
    assert!( implements!( tuple => X3BasicInterface ) );
    assert!( implements!( tuple => X3CanonicalInterface ) );
  }

  //

  fn impls_x3_for_tuple_with_generics()
  {
    let tuple = TestX3TupleWithGenerics( 1, 2, 3 );
    assert!( implements!( tuple => X3NominalInterface ) );
    assert!( implements!( tuple => X3Interface ) );
    assert!( implements!( tuple => X3BasicInterface ) );
    assert!( implements!( tuple => X3CanonicalInterface ) );
  }

  //

  fn impls_x3_for_named()
  {
    let named = TestX3Named{ x : 1, y : 2, z : 3 };
    assert!( implements!( named => X3NominalInterface ) );
    assert!( implements!( named => X3Interface ) );
    assert!( implements!( named => X3BasicInterface ) );
    assert!( implements!( named => X3CanonicalInterface ) );
  }

  //

  fn impls_x3_for_named_with_generics()
  {
    let named = TestX3NamedWithGenerics{ x : 1, y : 2, z : 3 };
    assert!( implements!( named => X3NominalInterface ) );
    assert!( implements!( named => X3Interface ) );
    assert!( implements!( named => X3BasicInterface ) );
    assert!( implements!( named => X3CanonicalInterface ) );
  }

  //

  fn impls_x4_for_tuple()
  {
    let tuple = TestX4Tuple( 1, 2, 3, 4 );
    assert!( implements!( tuple => X4NominalInterface ) );
    assert!( implements!( tuple => X4Interface ) );
    assert!( implements!( tuple => X4BasicInterface ) );
    assert!( implements!( tuple => X4CanonicalInterface ) );
  }

  //

  fn impls_x4_for_tuple_with_generics()
  {
    let tuple = TestX4TupleWithGenerics( 1, 2, 3, 4 );
    assert!( implements!( tuple => X4NominalInterface ) );
    assert!( implements!( tuple => X4Interface ) );
    assert!( implements!( tuple => X4BasicInterface ) );
    assert!( implements!( tuple => X4CanonicalInterface ) );
  }

  //

  fn impls_x4_for_named()
  {
    let named = TestX4Named{ x : 1, y : 2, z : 3, w : 4 };
    assert!( implements!( named => X4NominalInterface ) );
    assert!( implements!( named => X4Interface ) );
    assert!( implements!( named => X4BasicInterface ) );
    assert!( implements!( named => X4CanonicalInterface ) );
  }

  //

  fn impls_x4_for_named_with_generics()
  {
    let named = TestX4NamedWithGenerics{ x : 1, y : 2, z : 3, w : 4 };
    assert!( implements!( named => X4NominalInterface ) );
    assert!( implements!( named => X4Interface ) );
    assert!( implements!( named => X4BasicInterface ) );
    assert!( implements!( named => X4CanonicalInterface ) );
  }
}

//

tests_index!
{
  impls_x2_for_tuple,
  impls_x2_for_tuple_with_generics,
  impls_x2_for_named,
  impls_x2_for_named_with_generics,
  impls_x3_for_tuple,
  impls_x3_for_tuple_with_generics,
  impls_x3_for_named,
  impls_x3_for_named_with_generics,
  impls_x4_for_tuple,
  impls_x4_for_tuple_with_generics,
  impls_x4_for_named,
  impls_x4_for_named_with_generics,
}
