#[ macro_export ]
macro_rules! x2_with_records_test_for
{

  () =>
  {
  };

  ( $type : ident $(, $( $tail : ident ),* )? ) =>
  {

    {
      type T = $type;
      println!( "Testing {}", stringify!( $type ) );

      /* test.case = "size"; */
      {
        assert_eq!( size_of::< cgmath::Vector2::< T > >(), size_of::< ( T, T ) >() );
        assert_eq!( size_of::< cgmath::Vector2::< T > >(), size_of::< [ T ; 2 ] >() );
      }

      /* test.case = "value of elements"; */
      {
        let got = cgmath::Vector2::< T >::make( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        assert_eq!( got.x, cast::< _, T >( 1 ).unwrap() );
        assert_eq!( got.y, cast::< _, T >( 2 ).unwrap() );
        assert_eq!( got._0(), cast::< _, T >( 1 ).unwrap() );
        assert_eq!( got._1(), cast::< _, T >( 2 ).unwrap() );
        assert_eq!( got.x(), cast::< _, T >( 1 ).unwrap() );
        assert_eq!( got.y(), cast::< _, T >( 2 ).unwrap() );
        assert_eq!( *got._0_ref(), cast::< _, T >( 1 ).unwrap() );
        assert_eq!( *got._1_ref(), cast::< _, T >( 2 ).unwrap() );
      }

      /* test.case = "set value of elements"; */
      {
        let mut got = cgmath::Vector2::< T >::make( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        assert_eq!( got._0(), cast::< _, T >( 1 ).unwrap() );
        assert_eq!( got._1(), cast::< _, T >( 2 ).unwrap() );
        assert_eq!( *got._0_mut(), cast::< _, T >( 1 ).unwrap() );
        assert_eq!( *got._1_mut(), cast::< _, T >( 2 ).unwrap() );
        *got._0_mut() = cast::< _, T >( 11 ).unwrap();
        *got._1_mut() = cast::< _, T >( 22 ).unwrap();
        assert_eq!( got._0(), cast::< _, T >( 11 ).unwrap() );
        assert_eq!( got._1(), cast::< _, T >( 22 ).unwrap() );
        assert_eq!( *got._0_mut(), cast::< _, T >( 11 ).unwrap() );
        assert_eq!( *got._1_mut(), cast::< _, T >( 22 ).unwrap() );
      }

      /* test.case = "make"; */
      {
        let got = cgmath::Vector2::< T >::make( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let exp = cgmath::Vector2::< T >{ x : cast::< _, T >( 1 ).unwrap(), y : cast::< _, T >( 2 ).unwrap() };
        assert_eq!( got, exp );
      }

      /* test.case = "clone_as_tuple"; */
      {
        let src = cgmath::Vector2::< T >::make( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.clone_as_tuple();
        let exp : ( T , T ) = ( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        assert_eq!( got, exp );
        assert!( !mem_same_ptrs( &got, &src ) ); /* qqq : discuss */
      }

      /* test.case = "clone_as_array"; */
      {
        let src = cgmath::Vector2::< T >::make( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.clone_as_array();
        let exp : [ T ; 2 ] = [ cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() ];
        assert_eq!( got, exp );
        assert!( !mem_same_ptrs( &got, &src ) );
      }

      /* test.case = "clone_as_canonical"; */
      {
        let src = cgmath::Vector2::< T >::make( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.clone_as_canonical();
        let exp = wmath::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        assert_eq!( got, exp );
        assert!( !mem_same_ptrs( &got, &src ) );
      }

      // --

      /* test.case = "as_tuple"; */
      {
        let src = cgmath::Vector2::< T >::make( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_tuple();
        let exp : ( T , T ) = ( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        assert_eq!( got, &exp );
        assert!( mem_same( got, &src ) ); /* qqq : discuss */
      }

      /* test.case = "as_array"; */
      {
        let src = cgmath::Vector2::< T >::make( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_array();
        let exp : [ T ; 2 ] = [ cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() ];
        assert_eq!( got, &exp );
        assert!( mem_same( got, &src ) );
      }

      /* test.case = "as_canonical"; */
      {
        let src = cgmath::Vector2::< T >::make( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_canonical();
        let exp = wmath::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        assert_eq!( got, &exp );
        assert!( mem_same( got, &src ) );
      }

      /* test.case = "as_slice"; */
      {
        let src = cgmath::Vector2::< T >::make( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_slice();
        let exp = &[ cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() ][ .. ];
        assert_eq!( got, exp );
      }

      // --

      /* test.case = "as_tuple_mut"; */
      {
        let mut src = cgmath::Vector2::< T >::make( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_tuple_mut();
        got.0 = cast::< _, T >( 11 ).unwrap();
        got.1 = cast::< _, T >( 22 ).unwrap();
        let exp = cgmath::Vector2::< T >::make( cast::< _, T >( 11 ).unwrap(), cast::< _, T >( 22 ).unwrap() );
        assert_eq!( &src, &exp );
      }

      /* test.case = "as_array"; */
      {
        let mut src = cgmath::Vector2::< T >::make( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_array_mut();
        got[ 0 ] = cast::< _, T >( 11 ).unwrap();
        got[ 1 ] = cast::< _, T >( 22 ).unwrap();
        let exp = cgmath::Vector2::< T >::make( cast::< _, T >( 11 ).unwrap(), cast::< _, T >( 22 ).unwrap() );
        assert_eq!( &src, &exp );
      }

      /* test.case = "as_canonical"; */
      {
        let mut src = cgmath::Vector2::< T >::make( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_canonical_mut();
        *got._0_mut() = cast::< _, T >( 11 ).unwrap();
        *got._1_mut() = cast::< _, T >( 22 ).unwrap();
        let exp = cgmath::Vector2::< T >::make( cast::< _, T >( 11 ).unwrap(), cast::< _, T >( 22 ).unwrap() );
        assert_eq!( &src, &exp );
      }

      /* test.case = "as_slice"; */
      {
        let mut src = cgmath::Vector2::< T >::make( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_slice_mut();
        got[ 0 ] = cast::< _, T >( 11 ).unwrap();
        got[ 1 ] = cast::< _, T >( 22 ).unwrap();
        let exp = cgmath::Vector2::< T >::make( cast::< _, T >( 11 ).unwrap(), cast::< _, T >( 22 ).unwrap() );
        assert_eq!( &src, &exp );
      }

      // --

      /* test.case = "operator add"; */
      {
        let src1 = cgmath::Vector2::< T >::make( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let src2 = cgmath::Vector2::< T >::make( cast::< _, T >( 2 ).unwrap(), cast::< _, T >( 3 ).unwrap() );
        let got = src1 + src2;
        let exp = cgmath::Vector2::< T >::make( cast::< _, T >( 3 ).unwrap(), cast::< _, T >( 5 ).unwrap() );
        // let exp = wmath::x2::< T >( cast::< _, T >( 3 ).unwrap(), cast::< _, T >( 5 ).unwrap() );
        assert_eq!( got, exp );
      }

    }

    $crate::x2_with_records_test_for!( $( $( $tail ),* )? );
  };

}