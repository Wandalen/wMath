//
// #[ macro_export ]
// macro_rules! Vec
// {
//   () =>
//   {
//     $Va $( :: $Vb )*
//   };
// }

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
        let got = $Va $( :: $Vb )* ::< T >::from2( val!( 1, 2 ) );
        assert_eq!( got.$_0, val!( 1 ) );
        assert_eq!( got.$_1, val!( 2 ) );
        assert_eq!( got._0(), val!( 1 ) );
        assert_eq!( got._1(), val!( 2 ) );
        assert_eq!( got.x(), val!( 1 ) );
        assert_eq!( got.y(), val!( 2 ) );
        assert_eq!( *got._0_ref(), val!( 1 ) );
        assert_eq!( *got._1_ref(), val!( 2 ) );
      }

      /* test.case = "set value of elements"; */
      {
        let mut got = $Va $( :: $Vb )* ::< T >::from2( val!( 1, 2 ) );
        assert_eq!( got._0(), val!( 1 ) );
        assert_eq!( got._1(), val!( 2 ) );
        assert_eq!( *got._0_mut(), val!( 1 ) );
        assert_eq!( *got._1_mut(), val!( 2 ) );
        *got._0_mut() = val!( 11 );
        *got._1_mut() = val!( 22 );
        assert_eq!( got._0(), val!( 11 ) );
        assert_eq!( got._1(), val!( 22 ) );
        assert_eq!( *got._0_mut(), val!( 11 ) );
        assert_eq!( *got._1_mut(), val!( 22 ) );
      }

      /* test.case = "make"; */
      {
        let got = $Va $( :: $Vb )* ::< T >::from2( val!( 1, 2 ) );
        let exp = $Va $( :: $Vb )* ::< T >{ $_0 : val!( 1 ), $_1 : val!( 2 ) };
        assert_eq!( got, exp );
      }

      /* test.case = "assign"; */
      {
        // dbg!( val!( 1, 2 ) );
        // let mut dst = X2::< T >::from2(( val!( 1, 2 ) ));
        let mut dst = X2::< T >::from2( val!( 1, 2 ) );
        let src = $Va $( :: $Vb )* ::< T >::from2( val!( 11, 22 ) );
        dst.assign( src );
        let exp = X2::< T >::from2( val!( 11, 22 ) );
        assert_eq!( dst, exp );
        let mut dst = $Va $( :: $Vb )* ::< T >::from2( val!( 1, 2 ) );
        let src = X2::< T >::from2( val!( 11, 22 ) );
        dst.assign( src );
        let exp = $Va $( :: $Vb )* ::< T >::from2( val!( 11, 22 ) );
        assert_eq!( dst, exp );
      }

      /* test.case = "clone_as_tuple"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( val!( 1, 2 ) );
        let got = src.clone_as_tuple();
        let exp : ( T , T ) = ( val!( 1 ), val!( 2 ) );
        assert_eq!( got, exp );
        assert!( !mem_same_ptrs( &got, &src ) ); /* qqq : discuss */
      }

      /* test.case = "clone_as_array"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( val!( 1, 2 ) );
        let got = src.clone_as_array();
        let exp : [ T ; 2 ] = [ val!( 1 ), val!( 2 ) ];
        assert_eq!( got, exp );
        assert!( !mem_same_ptrs( &got, &src ) );
      }

      /* test.case = "clone_as_canonical"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( val!( 1, 2 ) );
        let got = src.clone_as_canonical();
        let exp = X2::< T >( val!( 1 ), val!( 2 ) );
        assert_eq!( got, exp );
        assert!( !mem_same_ptrs( &got, &src ) );
      }

      // --

      /* test.case = "as_tuple"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( val!( 1, 2 ) );
        let got = src.as_tuple();
        let exp : ( T , T ) = ( val!( 1 ), val!( 2 ) );
        assert_eq!( got, &exp );
        assert!( mem_same( got, &src ) ); /* qqq : discuss */
      }

      /* test.case = "as_array"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( val!( 1, 2 ) );
        let got = src.as_array();
        let exp : [ T ; 2 ] = [ val!( 1 ), val!( 2 ) ];
        assert_eq!( got, &exp );
        assert!( mem_same( got, &src ) );
      }

      /* test.case = "as_canonical"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( val!( 1, 2 ) );
        let got = src.as_canonical();
        let exp = X2::< T >( val!( 1 ), val!( 2 ) );
        assert_eq!( got, &exp );
        assert!( mem_same( got, &src ) );
      }

      /* test.case = "as_slice"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( val!( 1, 2 ) );
        let got = src.as_slice();
        let exp = &[ val!( 1 ), val!( 2 ) ][ .. ];
        assert_eq!( got, exp );
      }

      // --

      /* test.case = "as_tuple_mut"; */
      {
        let mut src = $Va $( :: $Vb )* ::< T >::from2( val!( 1, 2 ) );
        let got = src.as_tuple_mut();
        got.0 = val!( 11 );
        got.1 = val!( 22 );
        let exp = $Va $( :: $Vb )* ::< T >::from2( val!( 11, 22 ) );
        assert_eq!( &src, &exp );
      }

      /* test.case = "as_array_mut"; */
      {
        let mut src = $Va $( :: $Vb )* ::< T >::from2( val!( 1, 2 ) );
        let got = src.as_array_mut();
        got[ 0 ] = val!( 11 );
        got[ 1 ] = val!( 22 );
        let exp = $Va $( :: $Vb )* ::< T >::from2( val!( 11, 22 ) );
        assert_eq!( &src, &exp );
      }

      /* test.case = "as_canonical_mut"; */
      {
        let mut src = $Va $( :: $Vb )* ::< T >::from2( val!( 1, 2 ) );
        let got = src.as_canonical_mut();
        *got._0_mut() = val!( 11 );
        *got._1_mut() = val!( 22 );
        let exp = $Va $( :: $Vb )* ::< T >::from2( val!( 11, 22 ) );
        assert_eq!( &src, &exp );
      }

      /* test.case = "as_slice_mut"; */
      {
        let mut src = $Va $( :: $Vb )* ::< T >::from2( val!( 1, 2 ) );
        let got = src.as_slice_mut();
        got[ 0 ] = val!( 11 );
        got[ 1 ] = val!( 22 );
        let exp = $Va $( :: $Vb )* ::< T >::from2( val!( 11, 22 ) );
        assert_eq!( &src, &exp );
      }

      /* test.case = "into Canonical from Vector"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( val!( 1, 2 ) );
        let got = X2::< T >::from2( src );
        let exp = X2::< T >::from2( val!( 1, 2 ) );
        assert_eq!( got, exp );
        let got : X2< T > = src.into2();
        let exp = X2::< T >::from2( val!( 1, 2 ) );
        assert_eq!( got, exp );
      }

      /* test.case = "into Vector from Canonical"; */
      {
        let src = X2::< T >::from2( val!( 1, 2 ) );
        let got = $Va $( :: $Vb )* ::< T >::from2( src );
        let exp = $Va $( :: $Vb )* ::< T >::from2( val!( 1, 2 ) );
        assert_eq!( got, exp );
        let got : $Va $( :: $Vb )* < T > = src.into2();
        let exp = $Va $( :: $Vb )* ::< T >::from2( val!( 1, 2 ) );
        assert_eq!( got, exp );
      }

      // --

      // /* test.case = "operator add"; */
      // {
      //   let src1 = $Va $( :: $Vb )* ::< T >::from2( val!( 1, 2 ) );
      //   let src2 = $Va $( :: $Vb )* ::< T >::make( val!( 2 ), val!( 3 ) );
      //   let got = src1 + src2;
      //   let exp = $Va $( :: $Vb )* ::< T >::make( val!( 3 ), val!( 5 ) );
      //   // let exp = X2::< T >( val!( 3 ), val!( 5 ) );
      //   assert_eq!( got, exp );
      // }

    }

    x2_with_records_test_for!( $Va $( :: $Vb )* , $_0, $_1 ; $( $( $tail ),* )? );
  };

}