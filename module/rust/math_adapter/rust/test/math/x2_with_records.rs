
///
/// Required to convert integets to floats.
///

#[ macro_export ]
macro_rules! val
{

  ( $val : expr ) =>
  {
    num_traits::cast::cast::< _, T >( $val ).unwrap()
  };

}

///
/// Tests for x2 in a fromat of structure with 2 fields.
///

#[ macro_export ]
macro_rules! x2_with_records_test_for
{

  ( $Vec : ident $( :: $Vec2 : ident )*, $_0 : ident, $_1 : ident ; ) =>
  {
  };

  ( $Vec : ident $( :: $Vec2 : ident )*, $_0 : ident, $_1 : ident ; $type : ident $(, $( $tail : ident ),* )? ) =>
  {

    {
      type T = $type;
      println!( "Testing {}", stringify!( $type ) );

      /* test.case = "size"; */
      {
        assert_eq!( size_of::< $Vec $( :: $Vec2 )* ::< T > >(), size_of::< ( T, T ) >() );
        assert_eq!( size_of::< $Vec $( :: $Vec2 )* ::< T > >(), size_of::< [ T ; 2 ] >() );
      }

      /* test.case = "value of elements"; */
      {
        let got = $Vec $( :: $Vec2 )* ::< T >::make( $crate::val!( 1 ), $crate::val!( 2 ) );
        assert_eq!( got.$_0, $crate::val!( 1 ) );
        assert_eq!( got.$_1, $crate::val!( 2 ) );
        assert_eq!( got._0(), $crate::val!( 1 ) );
        assert_eq!( got._1(), $crate::val!( 2 ) );
        assert_eq!( got.x(), $crate::val!( 1 ) );
        assert_eq!( got.y(), $crate::val!( 2 ) );
        assert_eq!( *got._0_ref(), $crate::val!( 1 ) );
        assert_eq!( *got._1_ref(), $crate::val!( 2 ) );
      }

      /* test.case = "set value of elements"; */
      {
        let mut got = $Vec $( :: $Vec2 )* ::< T >::make( $crate::val!( 1 ), $crate::val!( 2 ) );
        assert_eq!( got._0(), $crate::val!( 1 ) );
        assert_eq!( got._1(), $crate::val!( 2 ) );
        assert_eq!( *got._0_mut(), $crate::val!( 1 ) );
        assert_eq!( *got._1_mut(), $crate::val!( 2 ) );
        *got._0_mut() = $crate::val!( 11 );
        *got._1_mut() = $crate::val!( 22 );
        assert_eq!( got._0(), $crate::val!( 11 ) );
        assert_eq!( got._1(), $crate::val!( 22 ) );
        assert_eq!( *got._0_mut(), $crate::val!( 11 ) );
        assert_eq!( *got._1_mut(), $crate::val!( 22 ) );
      }

      /* test.case = "make"; */
      {
        let got = $Vec $( :: $Vec2 )* ::< T >::make( $crate::val!( 1 ), $crate::val!( 2 ) );
        let exp = $Vec $( :: $Vec2 )* ::< T >{ $_0 : $crate::val!( 1 ), $_1 : $crate::val!( 2 ) };
        assert_eq!( got, exp );
      }

      /* test.case = "clone_as_tuple"; */
      {
        let src = $Vec $( :: $Vec2 )* ::< T >::make( $crate::val!( 1 ), $crate::val!( 2 ) );
        let got = src.clone_as_tuple();
        let exp : ( T , T ) = ( $crate::val!( 1 ), $crate::val!( 2 ) );
        assert_eq!( got, exp );
        assert!( !mem_same_ptrs( &got, &src ) ); /* qqq : discuss */
      }

      /* test.case = "clone_as_array"; */
      {
        let src = $Vec $( :: $Vec2 )* ::< T >::make( $crate::val!( 1 ), $crate::val!( 2 ) );
        let got = src.clone_as_array();
        let exp : [ T ; 2 ] = [ $crate::val!( 1 ), $crate::val!( 2 ) ];
        assert_eq!( got, exp );
        assert!( !mem_same_ptrs( &got, &src ) );
      }

      /* test.case = "clone_as_canonical"; */
      {
        let src = $Vec $( :: $Vec2 )* ::< T >::make( $crate::val!( 1 ), $crate::val!( 2 ) );
        let got = src.clone_as_canonical();
        let exp = math_adapter::x2::< T >( $crate::val!( 1 ), $crate::val!( 2 ) );
        assert_eq!( got, exp );
        assert!( !mem_same_ptrs( &got, &src ) );
      }

      // --

      /* test.case = "as_tuple"; */
      {
        let src = $Vec $( :: $Vec2 )* ::< T >::make( $crate::val!( 1 ), $crate::val!( 2 ) );
        let got = src.as_tuple();
        let exp : ( T , T ) = ( $crate::val!( 1 ), $crate::val!( 2 ) );
        assert_eq!( got, &exp );
        assert!( mem_same( got, &src ) ); /* qqq : discuss */
      }

      /* test.case = "as_array"; */
      {
        let src = $Vec $( :: $Vec2 )* ::< T >::make( $crate::val!( 1 ), $crate::val!( 2 ) );
        let got = src.as_array();
        let exp : [ T ; 2 ] = [ $crate::val!( 1 ), $crate::val!( 2 ) ];
        assert_eq!( got, &exp );
        assert!( mem_same( got, &src ) );
      }

      /* test.case = "as_canonical"; */
      {
        let src = $Vec $( :: $Vec2 )* ::< T >::make( $crate::val!( 1 ), $crate::val!( 2 ) );
        let got = src.as_canonical();
        let exp = math_adapter::x2::< T >( $crate::val!( 1 ), $crate::val!( 2 ) );
        assert_eq!( got, &exp );
        assert!( mem_same( got, &src ) );
      }

      /* test.case = "as_slice"; */
      {
        let src = $Vec $( :: $Vec2 )* ::< T >::make( $crate::val!( 1 ), $crate::val!( 2 ) );
        let got = src.as_slice();
        let exp = &[ $crate::val!( 1 ), $crate::val!( 2 ) ][ .. ];
        assert_eq!( got, exp );
      }

      // --

      /* test.case = "as_tuple_mut"; */
      {
        let mut src = $Vec $( :: $Vec2 )* ::< T >::make( $crate::val!( 1 ), $crate::val!( 2 ) );
        let got = src.as_tuple_mut();
        got.0 = $crate::val!( 11 );
        got.1 = $crate::val!( 22 );
        let exp = $Vec $( :: $Vec2 )* ::< T >::make( $crate::val!( 11 ), $crate::val!( 22 ) );
        assert_eq!( &src, &exp );
      }

      /* test.case = "as_array"; */
      {
        let mut src = $Vec $( :: $Vec2 )* ::< T >::make( $crate::val!( 1 ), $crate::val!( 2 ) );
        let got = src.as_array_mut();
        got[ 0 ] = $crate::val!( 11 );
        got[ 1 ] = $crate::val!( 22 );
        let exp = $Vec $( :: $Vec2 )* ::< T >::make( $crate::val!( 11 ), $crate::val!( 22 ) );
        assert_eq!( &src, &exp );
      }

      /* test.case = "as_canonical"; */
      {
        let mut src = $Vec $( :: $Vec2 )* ::< T >::make( $crate::val!( 1 ), $crate::val!( 2 ) );
        let got = src.as_canonical_mut();
        *got._0_mut() = $crate::val!( 11 );
        *got._1_mut() = $crate::val!( 22 );
        let exp = $Vec $( :: $Vec2 )* ::< T >::make( $crate::val!( 11 ), $crate::val!( 22 ) );
        assert_eq!( &src, &exp );
      }

      /* test.case = "as_slice"; */
      {
        let mut src = $Vec $( :: $Vec2 )* ::< T >::make( $crate::val!( 1 ), $crate::val!( 2 ) );
        let got = src.as_slice_mut();
        got[ 0 ] = $crate::val!( 11 );
        got[ 1 ] = $crate::val!( 22 );
        let exp = $Vec $( :: $Vec2 )* ::< T >::make( $crate::val!( 11 ), $crate::val!( 22 ) );
        assert_eq!( &src, &exp );
      }

      // --

      // /* test.case = "operator add"; */
      // {
      //   let src1 = $Vec $( :: $Vec2 )* ::< T >::make( $crate::val!( 1 ), $crate::val!( 2 ) );
      //   let src2 = $Vec $( :: $Vec2 )* ::< T >::make( $crate::val!( 2 ), $crate::val!( 3 ) );
      //   let got = src1 + src2;
      //   let exp = $Vec $( :: $Vec2 )* ::< T >::make( $crate::val!( 3 ), $crate::val!( 5 ) );
      //   // let exp = math_adapter::x2::< T >( $crate::val!( 3 ), $crate::val!( 5 ) );
      //   assert_eq!( got, exp );
      // }

    }

    $crate::x2_with_records_test_for!( $Vec $( :: $Vec2 )* , $_0, $_1 ; $( $( $tail ),* )? );
  };

}