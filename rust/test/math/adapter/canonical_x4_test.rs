use math_adapter::X4;
use super::*;

//

macro_rules! canonical_test_for_int
{

  () =>
  {
  };

  ( $type : path $(, $( $tail : tt ),* )? ) =>
  {{
    type T = $type;
    println!( "canonical_test_for_int {}", stringify!( $type ) );

    /* test.case = "make_nan"; */
    {
      let got = X4::< T >::make_nan();
      let exp = X4::< T >( num!( 0 ), num!( 0 ), num!( 0 ), num!( 0 ) );
      a_id!( got, exp );
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

  ( $type : path $(, $( $tail : tt ),* )? ) =>
  {{
    type T = $type;
    println!( "canonical_test_for_float {}", stringify!( $type ) );

    /* test.case = "make_nan"; */
    {
      let got = X4::< T >::make_nan();
      a_true!( got._0().is_nan() );
      a_true!( got._1().is_nan() );
      a_true!( got._2().is_nan() );
      a_true!( got._3().is_nan() );
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

  ( $type : path $(, $( $tail : tt ),* )? ) =>
  {{
    type T = $type;
    println!( "canonical_test_for_number {}", stringify!( $type ) );

    /* test.case = "size"; */
    {
      a_id!( size_of::< X4::< T > >(), size_of::< ( T, T, T, T ) >() );
      a_id!( size_of::< X4::< T > >(), size_of::< [ T ; 4 ] >() );
    }

    /* test.case = "from2 / into2 itself"; */
    {
      let src = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      let got : X4< T > = src.into2();
      a_id!( got, src );
      let got = X4::< T >::from2( src );
      a_id!( got, src );
    }

    /* test.case = "from2 / into2 tuple"; */
    {
      let src = ( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      let got : X4< T > = src.into2();
      let exp = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      a_id!( got, exp );
      let got = X4::< T >::from2( src );
      let exp = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      a_id!( got, exp );
    }

    /* test.case = "from2 / into2 array"; */
    {
      let src = [ num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) ];
      let got : X4< T > = src.into2();
      let exp = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      a_id!( got, exp );
      let got = X4::< T >::from2( src );
      let exp = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      a_id!( got, exp );
    }

    /* test.case = "from2 / into2 slice"; */
    {
      let _src = [ num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) ];
      let src = &_src[ .. ];
      let got : X4< T > = src.into2();
      let exp = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      a_id!( got, exp );
      let got = X4::< T >::from2( src );
      let exp = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      a_id!( got, exp );
    }

    /* test.case = "value of elements"; */
    {
      let got = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      a_id!( got.0, num!( 1 ) );
      a_id!( got.1, num!( 2 ) );
      a_id!( got.2, num!( 3 ) );
      a_id!( got.3, num!( 4 ) );
      a_id!( got._0(), num!( 1 ) );
      a_id!( got._1(), num!( 2 ) );
      a_id!( got._2(), num!( 3 ) );
      a_id!( got._3(), num!( 4 ) );
      a_id!( got.x(), num!( 1 ) );
      a_id!( got.y(), num!( 2 ) );
      a_id!( got.z(), num!( 3 ) );
      a_id!( got.w(), num!( 4 ) );
      a_id!( *got._0_ref(), num!( 1 ) );
      a_id!( *got._1_ref(), num!( 2 ) );
      a_id!( *got._2_ref(), num!( 3 ) );
      a_id!( *got._3_ref(), num!( 4 ) );
    }

    /* test.case = "set value of elements"; */
    {
      let mut got = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      a_id!( got._0(), num!( 1 ) );
      a_id!( got._1(), num!( 2 ) );
      a_id!( got._2(), num!( 3 ) );
      a_id!( got._3(), num!( 4 ) );
      a_id!( *got._0_mut(), num!( 1 ) );
      a_id!( *got._1_mut(), num!( 2 ) );
      a_id!( *got._2_mut(), num!( 3 ) );
      a_id!( *got._3_mut(), num!( 4 ) );
      *got._0_mut() = num!( 11 );
      *got._1_mut() = num!( 22 );
      *got._2_mut() = num!( 33 );
      *got._3_mut() = num!( 44 );
      a_id!( got._0(), num!( 11 ) );
      a_id!( got._1(), num!( 22 ) );
      a_id!( got._2(), num!( 33 ) );
      a_id!( got._3(), num!( 44 ) );
      a_id!( *got._0_mut(), num!( 11 ) );
      a_id!( *got._1_mut(), num!( 22 ) );
      a_id!( *got._2_mut(), num!( 33 ) );
      a_id!( *got._3_mut(), num!( 44 ) );
    }

    /* test.case = "make"; */
    {
      let got = X4::< T >::make( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      let exp = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      a_id!( got, exp );
    }

    /* test.case = "make_default"; */
    {
      let got = X4::< T >::make_default();
      let exp = X4::< T >( num!( 0 ), num!( 0 ), num!( 0 ), num!( 0 ) );
      a_id!( got, exp );
    }

    /* test.case = "clone_as_tuple"; */
    {
      let src = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      let got = src.clone_as_tuple();
      let exp : ( T, T, T, T ) = ( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      a_id!( got, exp );
      a_true!( !mem::same_ptr( &got, &src ) ); /* qqq : discuss postfix */
    }

    /* test.case = "clone_as_array"; */
    {
      let src = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      let got = src.clone_as_array();
      let exp : [ T ; 4 ] = [ num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) ];
      a_id!( got, exp );
      a_true!( !mem::same_ptr( &got, &src ) );
    }

    /* test.case = "clone_as_canonical"; */
    {
      let src = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      let got = src.clone_as_canonical();
      let exp = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      a_id!( got, exp );
      a_true!( !mem::same_ptr( &got, &src ) );
    }

    // --

    /* test.case = "as_tuple"; */
    {
      let src = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      let got = src.as_tuple();
      let exp : ( T, T, T, T ) = ( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      a_id!( got, &exp );
      a_true!( mem::same_region( got, &src ) ); /* qqq : discuss */
    }

    /* test.case = "as_array"; */
    {
      let src = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      let got = src.as_array();
      let exp : [ T ; 4 ] = [ num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) ];
      a_id!( got, &exp );
      a_true!( mem::same_region( got, &src ) );
    }

    /* test.case = "as_canonical"; */
    {
      let src = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      let got = src.as_canonical();
      let exp = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      a_id!( got, &exp );
      a_true!( mem::same_region( got, &src ) );
    }

    /* test.case = "as_slice"; */
    {
      let src = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      let got = src.as_slice();
      let exp = &[ num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) ][ .. ];
      a_id!( got, exp );
    }

    // --

    /* test.case = "as_tuple_mut"; */
    {
      let mut src = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      let got = src.as_tuple_mut();
      got.0 = num!( 11 );
      got.1 = num!( 22 );
      got.2 = num!( 33 );
      got.3 = num!( 44 );
      let exp = X4::< T >( num!( 11 ), num!( 22 ), num!( 33 ), num!( 44 ) );
      a_id!( src, exp );
    }

    /* test.case = "as_array_mut"; */
    {
      let mut src = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      let got = src.as_array_mut();
      got[ 0 ] = num!( 11 );
      got[ 1 ] = num!( 22 );
      got[ 2 ] = num!( 33 );
      got[ 3 ] = num!( 44 );
      let exp = X4::< T >( num!( 11 ), num!( 22 ), num!( 33 ), num!( 44 ) );
      a_id!( src, exp );
    }

    /* test.case = "as_canonical_mut"; */
    {
      let mut src = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      let got = src.as_canonical_mut();
      *got._0_mut() = num!( 11 );
      *got._1_mut() = num!( 22 );
      *got._2_mut() = num!( 33 );
      *got._3_mut() = num!( 44 );
      let exp = X4::< T >( num!( 11 ), num!( 22 ), num!( 33 ), num!( 44 ) );
      a_id!( src, exp );
    }

    /* test.case = "as_slice_mut"; */
    {
      let mut src = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      let got = src.as_slice_mut();
      got[ 0 ] = num!( 11 );
      got[ 1 ] = num!( 22 );
      got[ 2 ] = num!( 33 );
      got[ 3 ] = num!( 44 );
      let exp = X4::< T >( num!( 11 ), num!( 22 ), num!( 33 ), num!( 44 ) );
      a_id!( src, exp );
    }

    /* test.case = "assign"; */
    {
      let mut src = X4::< T >( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      let src2 = X4::< T >( num!( 11 ), num!( 22 ), num!( 33 ), num!( 44 ) );
      src.assign( src2 );
      let exp = X4::< T >( num!( 11 ), num!( 22 ), num!( 33 ), num!( 44 ) );
      a_id!( src, exp );
    }

    /* test.case = "debug format"; */
    {
      let src = X4::make( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      let got = format!( "{:?}", src );
      let exp = format!( "X4< {} >( {:?}, {:?}, {:?}, {:?} )", stringify!( $type ), src._0(), src._1(), src._2(), src._3() );
      a_id!( got, exp );
    }

    /* test.case = "format"; */
    {
      let src = X4::make( num!( 1 ), num!( 2 ), num!( 3 ), num!( 4 ) );
      let got = format!( "{}", src );
      let exp = format!( "X4< {} >( {}, {}, {}, {} )", stringify!( $type ), src._0(), src._1(), src._2(), src._3() );
      a_id!( got, exp );
    }

    canonical_test_for_number!( $( $( $tail ),* )? );
  }};

}

//

tests_impls!
{
  ///
  /// One test should be ordinary to exclude possibility of problems with macro.
  ///

  fn basic()
  {
    type T = i8;

    /* test.case = "size"; */
    {
      a_id!( size_of::< X4::< T > >(), size_of::< ( T, T, T, T ) >() );
      a_id!( size_of::< X4::< T > >(), size_of::< [ T ; 4 ] >() );
      a_id!( size_of::< X4::< T > >(), 4 );
    }

    /* test.case = "value of elements"; */
    {
      let got = X4::< i8 >( 1, 2, 3, 4 );
      a_id!( got.0, 1 );
      a_id!( got.1, 2 );
      a_id!( got.2, 3 );
      a_id!( got.3, 4 );
      a_id!( got._0(), 1 );
      a_id!( got._1(), 2 );
      a_id!( got._2(), 3 );
      a_id!( got._3(), 4 );
    }

  }

  ///
  /// Basic test of canonical X2 vector.
  ///

  fn canonical()
  {
    math_adapter::for_each_int!( canonical_test_for_int );
    math_adapter::for_each_float!( canonical_test_for_float );
    math_adapter::for_each_number!( canonical_test_for_number );
  }

  ///
  /// Operations without dereferencing.
  ///

  fn operation()
  {
    #[ cfg( any( feature = "cgmath_ops", feature = "nalgebra_ops" ) ) ]
    {
      type T = i8;

      /* test.case = "neg"; */
      {
        let src1 = X4::< T >::make( 4, 3, 3, 2 );
        let got = - src1;
        let exp = X4::< T >::make( -4, -3, -3, -2 );
        a_id!( got, exp );
      }

      /* test.case = "add"; */
      {
        let src1 = X4::< T >::make( 4, 3, 3, 2 );
        let src2 = X4::< T >::make( 2, 1, 1, 2 );
        let got = src1 + src2;
        let exp = X4::< T >::make( 6, 4, 4, 4 );
        a_id!( got, exp );
      }

      /* test.case = "sub"; */
      {
        let src1 = X4::< T >::make( 4, 3, 3, 2 );
        let src2 = X4::< T >::make( 1, 2, 2, -2 );
        let got = src1 - src2;
        let exp = X4::< T >::make( 3, 1, 1, 4 );
        a_id!( got, exp );
      }
    }
  }
}

//

tests_index!
{
  basic,
  canonical,
  operation,
}
