use test_tools::*;
use math_adapter::X2;
use super::*;

#[ derive( VectorInterfaces ) ]
struct TestTuple( u32, u32 );

#[ derive( VectorInterfaces ) ]
struct TestTupleWithGenerics< S : ScalarInterface >( S, S );

#[ derive( VectorInterfaces ) ]
struct TestNamed{ x : u32, y : u32 }

#[ derive( VectorInterfaces ) ]
struct TestNamedWithGenerics< S : ScalarInterface >{ x : S, y : S }

tests_impls!
{
  fn impls_for_tuple()
  {
    let tuple = TestTuple( 1, 2 );
    assert!( implements!( tuple => X2NominalInterface ) );
    assert!( implements!( tuple => X2Interface ) );
    assert!( implements!( tuple => X2BasicInterface ) );
    assert!( implements!( tuple => X2CanonicalInterface ) );
  }

  //

  fn impls_for_tuple_with_generics()
  {
    let tuple = TestTupleWithGenerics( 1, 2 );
    assert!( implements!( tuple => X2NominalInterface ) );
    assert!( implements!( tuple => X2Interface ) );
    assert!( implements!( tuple => X2BasicInterface ) );
    assert!( implements!( tuple => X2CanonicalInterface ) );
  }

  //

  fn impls_for_named()
  {
    let named = TestNamed{ x : 1, y : 2 };
    assert!( implements!( named => X2NominalInterface ) );
    assert!( implements!( named => X2Interface ) );
    assert!( implements!( named => X2BasicInterface ) );
    assert!( implements!( named => X2CanonicalInterface ) );
  }

  //

  fn impls_for_named_with_generics()
  {
    let named = TestNamedWithGenerics{ x : 1, y : 2 };
    assert!( implements!( named => X2NominalInterface ) );
    assert!( implements!( named => X2Interface ) );
    assert!( implements!( named => X2BasicInterface ) );
    assert!( implements!( named => X2CanonicalInterface ) );
  }
}

//

tests_index!
{
  impls_for_tuple,
  impls_for_tuple_with_generics,
  impls_for_named,
  impls_for_named_with_generics,
}
