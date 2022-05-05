use wtest_basic::*;
use core::mem::size_of;
use math_adapter::prelude::*;
use math_adapter::X2;
use crate::test_tools::*;
use crate::num;

///
/// One test should be ordinary to exclude possibility of problems with macro.
///

fn _basic_test()
{
  type T = i8;

  /* test.case = "size"; */
  {
    assert_eq!( size_of::< X2::< T > >(), size_of::< ( T, T ) >() );
    assert_eq!( size_of::< X2::< T > >(), size_of::< [ T ; 2 ] >() );
    assert_eq!( size_of::< X2::< T > >(), 2 );
  }

  /* test.case = "value of elements"; */
  {
    let got = X2::< i8 >( 1, 2 );
    assert_eq!( got.0, 1 );
    assert_eq!( got.1, 2 );
    assert_eq!( got._0(), 1 );
    assert_eq!( got._1(), 2 );
  }

  // /* test.case = "operator add"; */
  // {
  //   let src1 = X2::< i8 >( 1, 2 );
  //   let src2 = X2::< i8 >( 2, 3 );
  //   let got = src1 + src2;
  //   let exp = X2::< i8 >( 3, 5 );
  //   assert_eq!( got, exp );
  // }

}

//

macro_rules! canonical_test_for_int
{

  () =>
  {
  };

  ( $type : ident $(, $( $tail : ident ),* )? ) =>
  {{
    type T = $type;
    println!( "canonical_test_for_int {}", stringify!( $type ) );

    /* test.case = "make_nan"; */
    {
      let got = X2::< T >::make_nan();
      let exp = X2::< T >( num!( 0 ), num!( 0 ) );
      assert_eq!( got, exp );
    }

    canonical_test_for_int!( $( $( $tail ),* )? );
  }}

}

//

macro_rules! canonical_test_for_float
{

  () =>
  {
  };

  ( $type : ident $(, $( $tail : ident ),* )? ) =>
  {{
    type T = $type;
    println!( "canonical_test_for_float {}", stringify!( $type ) );

    /* test.case = "make_nan"; */
    {
      let got = X2::< T >::make_nan();
      assert!( got._0().is_nan() );
      assert!( got._1().is_nan() );
    }

    canonical_test_for_float!( $( $( $tail ),* )? );
  }}

}

//

macro_rules! canonical_test_for_number
{

  () =>
  {
  };

  ( $type : ident $(, $( $tail : ident ),* )? ) =>
  {{
    type T = $type;
    println!( "canonical_test_for_number {}", stringify!( $type ) );

    // assert!( false );

    /* test.case = "size"; */
    {
      assert_eq!( size_of::< X2::< T > >(), size_of::< ( T, T ) >() );
      assert_eq!( size_of::< X2::< T > >(), size_of::< [ T ; 2 ] >() );
    }

    /* test.case = "from / into itself"; */
    {
      let src = X2::< T >( num!( 1 ), num!( 2 ) );
      let got : X2< T > = src.into2();
      assert_eq!( got, src );
      let got = X2::< T >::from2( src );
      assert_eq!( got, src );
    }

    /* test.case = "from / into tuple"; */
    {
      let src = ( num!( 1 ), num!( 2 ) );
      let got : X2< T > = src.into2();
      let exp = X2::< T >( num!( 1 ), num!( 2 ) );
      assert_eq!( got, exp );
      let got = X2::< T >::from2( src );
      let exp = X2::< T >( num!( 1 ), num!( 2 ) );
      assert_eq!( got, exp );
    }

    /* test.case = "from / into array"; */
    {
      let src = [ num!( 1 ), num!( 2 ) ];
      let got : X2< T > = src.into2();
      let exp = X2::< T >( num!( 1 ), num!( 2 ) );
      assert_eq!( got, exp );
      let got = X2::< T >::from2( src );
      let exp = X2::< T >( num!( 1 ), num!( 2 ) );
      assert_eq!( got, exp );
    }

    /* test.case = "from / into slice"; */
    {
      let _src = [ num!( 1 ), num!( 2 ) ];
      let src = &_src[ .. ];
      let got : X2< T > = src.into2();
      let exp = X2::< T >( num!( 1 ), num!( 2 ) );
      assert_eq!( got, exp );
      let got = X2::< T >::from2( src );
      let exp = X2::< T >( num!( 1 ), num!( 2 ) );
      assert_eq!( got, exp );
    }

    /* test.case = "value of elements"; */
    {
      let got = X2::< T >( num!( 1 ), num!( 2 ) );
      assert_eq!( got.0, num!( 1 ) );
      assert_eq!( got.1, num!( 2 ) );
      assert_eq!( got._0(), num!( 1 ) );
      assert_eq!( got._1(), num!( 2 ) );
      assert_eq!( got.x(), num!( 1 ) );
      assert_eq!( got.y(), num!( 2 ) );
      assert_eq!( *got._0_ref(), num!( 1 ) );
      assert_eq!( *got._1_ref(), num!( 2 ) );
    }

    /* test.case = "set value of elements"; */
    {
      let mut got = X2::< T >( num!( 1 ), num!( 2 ) );
      assert_eq!( got._0(), num!( 1 ) );
      assert_eq!( got._1(), num!( 2 ) );
      assert_eq!( *got._0_mut(), num!( 1 ) );
      assert_eq!( *got._1_mut(), num!( 2 ) );
      *got._0_mut() = num!( 11 );
      *got._1_mut() = num!( 22 );
      assert_eq!( got._0(), num!( 11 ) );
      assert_eq!( got._1(), num!( 22 ) );
      assert_eq!( *got._0_mut(), num!( 11 ) );
      assert_eq!( *got._1_mut(), num!( 22 ) );
    }

    /* test.case = "make"; */
    {
      let got = X2::< T >::make( num!( 1 ), num!( 2 ) );
      let exp = X2::< T >( num!( 1 ), num!( 2 ) );
      assert_eq!( got, exp );
    }

    /* test.case = "make_default"; */
    {
      let got = X2::< T >::make_default();
      let exp = X2::< T >( num!( 0 ), num!( 0 ) );
      assert_eq!( got, exp );
      let got : X2::< T > = Default::default();
      let exp = X2::< T >( num!( 0 ), num!( 0 ) );
      assert_eq!( got, exp );
    }

    /* test.case = "make_nan"; */
    {
      let _got = X2::< T >::make_nan();
      // let exp = X2::< T >( num!( 0 ), num!( 0 ) );
      // assert_eq!( got, exp );
    }

    /* test.case = "clone_as_tuple"; */
    {
      let src = X2::< T >( num!( 1 ), num!( 2 ) );
      let got = src.clone_as_tuple();
      let exp : ( T , T ) = ( num!( 1 ), num!( 2 ) );
      assert_eq!( got, exp );
      assert!( !mem_same_ptr( &got, &src ) ); /* qqq : discuss postfix */
    }

    /* test.case = "clone_as_array"; */
    {
      let src = X2::< T >( num!( 1 ), num!( 2 ) );
      let got = src.clone_as_array();
      let exp : [ T ; 2 ] = [ num!( 1 ), num!( 2 ) ];
      assert_eq!( got, exp );
      assert!( !mem_same_ptr( &got, &src ) );
    }

    /* test.case = "clone_as_canonical"; */
    {
      let src = X2::< T >( num!( 1 ), num!( 2 ) );
      let got = src.clone_as_canonical();
      let exp = X2::< T >( num!( 1 ), num!( 2 ) );
      assert_eq!( got, exp );
      assert!( !mem_same_ptr( &got, &src ) );
    }

    // --

    /* test.case = "as_tuple"; */
    {
      let src = X2::< T >( num!( 1 ), num!( 2 ) );
      let got = src.as_tuple();
      let exp : ( T , T ) = ( num!( 1 ), num!( 2 ) );
      assert_eq!( got, &exp );
      assert!( mem_same_region( got, &src ) ); /* qqq : discuss */
    }

    /* test.case = "as_array"; */
    {
      let src = X2::< T >( num!( 1 ), num!( 2 ) );
      let got = src.as_array();
      let exp : [ T ; 2 ] = [ num!( 1 ), num!( 2 ) ];
      assert_eq!( got, &exp );
      assert!( mem_same_region( got, &src ) );
    }

    /* test.case = "as_canonical"; */
    {
      let src = X2::< T >( num!( 1 ), num!( 2 ) );
      let got = src.as_canonical();
      let exp = X2::< T >( num!( 1 ), num!( 2 ) );
      assert_eq!( got, &exp );
      assert!( mem_same_region( got, &src ) );
    }

    /* test.case = "as_slice"; */
    {
      let src = X2::< T >( num!( 1 ), num!( 2 ) );
      let got = src.as_slice();
      let exp = &[ num!( 1 ), num!( 2 ) ][ .. ];
      assert_eq!( got, exp );
    }

    // --

    /* test.case = "as_tuple_mut"; */
    {
      let mut src = X2::< T >( num!( 1 ), num!( 2 ) );
      let got = src.as_tuple_mut();
      got.0 = num!( 11 );
      got.1 = num!( 22 );
      let exp = X2::< T >( num!( 11 ), num!( 22 ) );
      assert_eq!( src, exp );
    }

    /* test.case = "as_array_mut"; */
    {
      let mut src = X2::< T >( num!( 1 ), num!( 2 ) );
      let got = src.as_array_mut();
      got[ 0 ] = num!( 11 );
      got[ 1 ] = num!( 22 );
      let exp = X2::< T >( num!( 11 ), num!( 22 ) );
      assert_eq!( src, exp );
    }

    /* test.case = "as_canonical_mut"; */
    {
      let mut src = X2::< T >( num!( 1 ), num!( 2 ) );
      let got = src.as_canonical_mut();
      *got._0_mut() = num!( 11 );
      *got._1_mut() = num!( 22 );
      let exp = X2::< T >( num!( 11 ), num!( 22 ) );
      assert_eq!( src, exp );
    }

    /* test.case = "as_slice_mut"; */
    {
      let mut src = X2::< T >( num!( 1 ), num!( 2 ) );
      let got = src.as_slice_mut();
      got[ 0 ] = num!( 11 );
      got[ 1 ] = num!( 22 );
      let exp = X2::< T >( num!( 11 ), num!( 22 ) );
      assert_eq!( src, exp );
    }

    /* test.case = "assign"; */
    {
      let mut src = X2::< T >( num!( 1 ), num!( 2 ) );
      let src2 = X2::< T >( num!( 11 ), num!( 22 ) );
      src.assign( src2 );
      let exp = X2::< T >( num!( 11 ), num!( 22 ) );
      assert_eq!( src, exp );
    }

    /* test.case = "debug format"; */
    {
      let src = X2::< T >::from2( num!( 1, 2 ) );
      let got = format!( "{:?}", src );
      let exp = format!( "X2< {} >( {:?}, {:?} )", stringify!( $type ), src._0(), src._1() );
      assert_eq!( got, exp );
    }

    /* test.case = "format"; */
    {
      let src = X2::< T >::from2( num!( 1, 2 ) );
      let got = format!( "{}", src );
      let exp = format!( "X2< {} >( {}, {} )", stringify!( $type ), src._0(), src._1() );
      assert_eq!( got, exp );
    }

    canonical_test_for_number!( $( $( $tail ),* )? );
  }};

}

///
/// Basic test of canonical X2 vector.
///

fn _canonical_test()
{
  math_adapter::for_each_int!( canonical_test_for_int );
  math_adapter::for_each_float!( canonical_test_for_float );
  math_adapter::for_each_number!( canonical_test_for_number );
  // canonical_test_for_int!( i8, i16, i32, i64, i128, u8, u16, u32, u64, u128 );
  // canonical_test_for_float!( f32, f64 );
  // canonical_test_for_number!( i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64 );
}

///
/// Operations without dereferencing.
///

fn _operation_test()
{
  #[ cfg( any( feature = "cgmath_ops", feature = "nalgebra_ops", feature = "default_ops" ) ) ]
  {
    type T = i8;

    /* test.case = "neg"; */
    {
      let src1 = X2::< T >::make( 4, 3 );
      let got = - src1;
      let exp = X2::< T >::make( -4, -3 );
      assert_eq!( got, exp );
    }

    /* test.case = "add"; */
    {
      let src1 = X2::< T >::make( 4, 3 );
      let src2 = X2::< T >::make( 2, 1 );
      let got = src1 + src2;
      let exp = X2::< T >::make( 6, 4 );
      assert_eq!( got, exp );
    }

    /* test.case = "sub"; */
    {
      let src1 = X2::< T >::make( 4, 3 );
      let src2 = X2::< T >::make( 1, 2 );
      let got = src1 - src2;
      let exp = X2::< T >::make( 3, 1 );
      assert_eq!( got, exp );
    }
  }
}

//

test_suite!
{
  basic_test,
  canonical_test,
  operation_test,
}
