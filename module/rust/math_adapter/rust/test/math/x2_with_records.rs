
///
/// Tests for X2 in a fromat of structure with 2 fields.
///

#[ macro_export ]
macro_rules! x2_with_records_test_for
{

  ( $Va : ident $( :: $Vb : ident )*, $_0 : ident, $_1 : ident ; ) =>
  {
  };

  ( $Va : ident $( :: $Vb : ident )*, $_0 : ident, $_1 : ident ; $type : ident $(, $( $tail : ident ),* )? ) =>
  {

    {
      type T = $type;
      println!( "Testing {}", stringify!( $type ) );

      /* test.case = "size"; */
      {
        assert_eq!( size_of::< $Va $( :: $Vb )* ::< T > >(), size_of::< ( T, T ) >() );
        assert_eq!( size_of::< $Va $( :: $Vb )* ::< T > >(), size_of::< [ T ; 2 ] >() );
      }

      /* test.case = "value of elements"; */
      {
        let got = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        assert_eq!( got.$_0, num!( 1 ) );
        assert_eq!( got.$_1, num!( 2 ) );
        assert_eq!( got._0(), num!( 1 ) );
        assert_eq!( got._1(), num!( 2 ) );
        assert_eq!( got.x(), num!( 1 ) );
        assert_eq!( got.y(), num!( 2 ) );
        assert_eq!( *got._0_ref(), num!( 1 ) );
        assert_eq!( *got._1_ref(), num!( 2 ) );
      }

      /* test.case = "set value of elements"; */
      {
        let mut got = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
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
        let got = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let exp = $Va $( :: $Vb )* ::< T >{ $_0 : num!( 1 ), $_1 : num!( 2 ) };
        assert_eq!( got, exp );
      }

      /* test.case = "assign"; */
      {
        // dbg!( num!( 1, 2 ) );
        // let mut dst = X2::< T >::from2(( num!( 1, 2 ) ));
        let mut dst = X2::< T >::from2( num!( 1, 2 ) );
        let src = $Va $( :: $Vb )* ::< T >::from2( num!( 11, 22 ) );
        dst.assign( src );
        let exp = X2::< T >::from2( num!( 11, 22 ) );
        assert_eq!( dst, exp );
        let mut dst = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let src = X2::< T >::from2( num!( 11, 22 ) );
        dst.assign( src );
        let exp = $Va $( :: $Vb )* ::< T >::from2( num!( 11, 22 ) );
        assert_eq!( dst, exp );
      }

      /* test.case = "clone_as_tuple"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = src.clone_as_tuple();
        let exp : ( T , T ) = ( num!( 1 ), num!( 2 ) );
        assert_eq!( got, exp );
        assert!( !mem_same_ptr( &got, &src ) ); /* qqq : discuss */
      }

      /* test.case = "clone_as_array"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = src.clone_as_array();
        let exp : [ T ; 2 ] = [ num!( 1 ), num!( 2 ) ];
        assert_eq!( got, exp );
        assert!( !mem_same_ptr( &got, &src ) );
      }

      /* test.case = "clone_as_canonical"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = src.clone_as_canonical();
        let exp = X2::< T >( num!( 1 ), num!( 2 ) );
        assert_eq!( got, exp );
        assert!( !mem_same_ptr( &got, &src ) );
      }

      // --

      /* test.case = "as_tuple"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = src.as_tuple();
        let exp : ( T , T ) = ( num!( 1 ), num!( 2 ) );
        assert_eq!( got, &exp );
        assert!( mem_same_region( got, &src ) ); /* qqq : discuss */
      }

      /* test.case = "as_array"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = src.as_array();
        let exp : [ T ; 2 ] = [ num!( 1 ), num!( 2 ) ];
        assert_eq!( got, &exp );
        assert!( mem_same_region( got, &src ) );
      }

      /* test.case = "as_canonical"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = src.as_canonical();
        let exp = X2::< T >( num!( 1 ), num!( 2 ) );
        assert_eq!( got, &exp );
        assert!( mem_same_region( got, &src ) );
      }

      /* test.case = "as_slice"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = src.as_slice();
        let exp = &[ num!( 1 ), num!( 2 ) ][ .. ];
        assert_eq!( got, exp );
      }

      // --

      /* test.case = "as_tuple_mut"; */
      {
        let mut src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = src.as_tuple_mut();
        got.0 = num!( 11 );
        got.1 = num!( 22 );
        let exp = $Va $( :: $Vb )* ::< T >::from2( num!( 11, 22 ) );
        assert_eq!( &src, &exp );
      }

      /* test.case = "as_array_mut"; */
      {
        let mut src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = src.as_array_mut();
        got[ 0 ] = num!( 11 );
        got[ 1 ] = num!( 22 );
        let exp = $Va $( :: $Vb )* ::< T >::from2( num!( 11, 22 ) );
        assert_eq!( &src, &exp );
      }

      /* test.case = "as_canonical_mut"; */
      {
        let mut src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = src.as_canonical_mut();
        *got._0_mut() = num!( 11 );
        *got._1_mut() = num!( 22 );
        let exp = $Va $( :: $Vb )* ::< T >::from2( num!( 11, 22 ) );
        assert_eq!( &src, &exp );
      }

      /* test.case = "as_slice_mut"; */
      {
        let mut src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = src.as_slice_mut();
        got[ 0 ] = num!( 11 );
        got[ 1 ] = num!( 22 );
        let exp = $Va $( :: $Vb )* ::< T >::from2( num!( 11, 22 ) );
        assert_eq!( &src, &exp );
      }

      /* test.case = "into Canonical from Vector"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = X2::< T >::from2( src );
        let exp = X2::< T >::from2( num!( 1, 2 ) );
        assert_eq!( got, exp );
        let got : X2< T > = src.into2();
        let exp = X2::< T >::from2( num!( 1, 2 ) );
        assert_eq!( got, exp );
      }

      /* test.case = "into Vector from Canonical"; */
      {
        let src = X2::< T >::from2( num!( 1, 2 ) );
        let got = $Va $( :: $Vb )* ::< T >::from2( src );
        let exp = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        assert_eq!( got, exp );
        let got : $Va $( :: $Vb )* < T > = src.into2();
        let exp = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        assert_eq!( got, exp );
      }

      // --

      /* test.case = "from / into tuple"; */
      {
        let src = ( num!( 1 ), num!( 2 ) );
        let got : $Va $( :: $Vb )*< T > = src.into2();
        let exp = $Va $( :: $Vb )*::< T >::make( num!( 1 ), num!( 2 ) );
        assert_eq!( got, exp );
        let got = $Va $( :: $Vb )*::< T >::from2( src );
        let exp = $Va $( :: $Vb )*::< T >::make( num!( 1 ), num!( 2 ) );
        assert_eq!( got, exp );
      }

      /* test.case = "from / into array"; */
      {
        let src = [ num!( 1 ), num!( 2 ) ];
        let got : $Va $( :: $Vb )*< T > = src.into2();
        let exp = $Va $( :: $Vb )*::< T >::make( num!( 1 ), num!( 2 ) );
        assert_eq!( got, exp );
        let got = $Va $( :: $Vb )*::< T >::from2( src );
        let exp = $Va $( :: $Vb )*::< T >::make( num!( 1 ), num!( 2 ) );
        assert_eq!( got, exp );
      }

      /* test.case = "from / into slice"; */
      {
        let _src = [ num!( 1 ), num!( 2 ) ];
        let src = &_src[ .. ];
        let got : $Va $( :: $Vb )*< T > = src.into2();
        let exp = $Va $( :: $Vb )*::< T >::make( num!( 1 ), num!( 2 ) );
        assert_eq!( got, exp );
        let got = $Va $( :: $Vb )*::< T >::from2( src );
        let exp = $Va $( :: $Vb )*::< T >::make( num!( 1 ), num!( 2 ) );
        assert_eq!( got, exp );
      }

      // --

    }

    x2_with_records_test_for!( $Va $( :: $Vb )* , $_0, $_1 ; $( $( $tail ),* )? );
  };

}