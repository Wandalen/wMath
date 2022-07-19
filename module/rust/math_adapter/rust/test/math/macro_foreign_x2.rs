#![ allow( unused_macros ) ]
#![ allow( unused_imports ) ]

/// Internal namespace.
pub( crate ) mod private
{

  ///
  /// For each number generate general tests for X2, scalar is number.
  ///

  macro_rules! macro_test_foreign_x2_number_for_each
  {
    (
      $( $Arg : tt )*
    ) =>
    {
      math_adapter::for_each_number!( crate::macro_foreign_x2::macro_test_foreign_x2_number where @Prefix( $( $Arg )* ) );
    }
  }

  ///
  /// General tests for X2, scalar is number.
  ///

  macro_rules! macro_test_foreign_x2_number
  {

    ( ( $Va : ident $( :: $Vb : ident )*, $_0 : ident, $_1 : ident ) ) =>
    {
    };

    ( ( $Va : ident $( :: $Vb : ident )*, $_0 : ident, $_1 : ident ) $Type : ident $(, $( $tail : ident ),* )? ) =>
    {{

      type T = $Type;
      println!( "Testing {}< {} >", stringify!( $Va$(::$Vb)* ), stringify!( $Type ) );

      /* test.case = "size"; */
      {
        a_id!( size_of::< $Va $( :: $Vb )* ::< T > >(), size_of::< ( T, T ) >() );
        a_id!( size_of::< $Va $( :: $Vb )* ::< T > >(), size_of::< [ T ; 2 ] >() );
      }

      /* test.case = "value of elements"; */
      {
        let got = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        a_id!( got.$_0, num!( 1 ) );
        a_id!( got.$_1, num!( 2 ) );
        a_id!( got._0(), num!( 1 ) );
        a_id!( got._1(), num!( 2 ) );
        a_id!( got.x(), num!( 1 ) );
        a_id!( got.y(), num!( 2 ) );
        a_id!( *got._0_ref(), num!( 1 ) );
        a_id!( *got._1_ref(), num!( 2 ) );
      }

      /* test.case = "set value of elements"; */
      {
        let mut got = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        a_id!( got._0(), num!( 1 ) );
        a_id!( got._1(), num!( 2 ) );
        a_id!( *got._0_mut(), num!( 1 ) );
        a_id!( *got._1_mut(), num!( 2 ) );
        *got._0_mut() = num!( 11 );
        *got._1_mut() = num!( 22 );
        a_id!( got._0(), num!( 11 ) );
        a_id!( got._1(), num!( 22 ) );
        a_id!( *got._0_mut(), num!( 11 ) );
        a_id!( *got._1_mut(), num!( 22 ) );
      }

      /* test.case = "make"; */
      {
        let got = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let exp = $Va $( :: $Vb )* ::< T >::make( num!( 1 ), num!( 2 ) );
        a_id!( got, exp );
      }

      /* test.case = "make_default"; */
      {
        let got = $Va $( :: $Vb )*::< T >::make_default();
        let exp = $Va $( :: $Vb )*::< T >::make( num!( 0 ), num!( 0 ) );
        a_id!( got, exp );
      }

      /* test.case = "assign"; */
      {
        let mut dst = X2::< T >::from2( num!( 1, 2 ) );
        let src = $Va $( :: $Vb )* ::< T >::from2( num!( 11, 22 ) );
        dst.assign( src );
        let exp = X2::< T >::from2( num!( 11, 22 ) );
        a_id!( dst, exp );
        let mut dst = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let src = X2::< T >::from2( num!( 11, 22 ) );
        dst.assign( src );
        let exp = $Va $( :: $Vb )* ::< T >::from2( num!( 11, 22 ) );
        a_id!( dst, exp );
      }

      /* test.case = "clone_as_tuple"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = src.clone_as_tuple();
        let exp : ( T , T ) = ( num!( 1 ), num!( 2 ) );
        a_id!( got, exp );
        a_true!( !mem_tools::mem::same_ptr( &got, &src ) ); /* qqq : discuss */
      }

      /* test.case = "clone_as_array"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = src.clone_as_array();
        let exp : [ T ; 2 ] = [ num!( 1 ), num!( 2 ) ];
        a_id!( got, exp );
        a_true!( !mem_tools::mem::same_ptr( &got, &src ) );
      }

      /* test.case = "clone_as_canonical"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = src.clone_as_canonical();
        let exp = X2::< T >( num!( 1 ), num!( 2 ) );
        a_id!( got, exp );
        a_true!( !mem_tools::mem::same_ptr( &got, &src ) );
      }

      // --

      /* test.case = "as_tuple"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = src.as_tuple();
        let exp : ( T , T ) = ( num!( 1 ), num!( 2 ) );
        a_id!( got, &exp );
        a_true!( mem_tools::mem::same_region( got, &src ) ); /* qqq : discuss */
      }

      /* test.case = "as_array"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = src.as_array();
        let exp : [ T ; 2 ] = [ num!( 1 ), num!( 2 ) ];
        a_id!( got, &exp );
        a_true!( mem_tools::mem::same_region( got, &src ) );
      }

      /* test.case = "as_canonical"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = src.as_canonical();
        let exp = X2::< T >( num!( 1 ), num!( 2 ) );
        a_id!( got, &exp );
        a_true!( mem_tools::mem::same_region( got, &src ) );
      }

      /* test.case = "as_slice"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = src.as_slice();
        let exp = &[ num!( 1 ), num!( 2 ) ][ .. ];
        a_id!( got, exp );
      }

      // --

      /* test.case = "as_tuple_mut"; */
      {
        let mut src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = src.as_tuple_mut();
        got.0 = num!( 11 );
        got.1 = num!( 22 );
        let exp = $Va $( :: $Vb )* ::< T >::from2( num!( 11, 22 ) );
        a_id!( &src, &exp );
      }

      /* test.case = "as_array_mut"; */
      {
        let mut src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = src.as_array_mut();
        got[ 0 ] = num!( 11 );
        got[ 1 ] = num!( 22 );
        let exp = $Va $( :: $Vb )* ::< T >::from2( num!( 11, 22 ) );
        a_id!( &src, &exp );
      }

      /* test.case = "as_canonical_mut"; */
      {
        let mut src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = src.as_canonical_mut();
        *got._0_mut() = num!( 11 );
        *got._1_mut() = num!( 22 );
        let exp = $Va $( :: $Vb )* ::< T >::from2( num!( 11, 22 ) );
        a_id!( &src, &exp );
      }

      /* test.case = "as_slice_mut"; */
      {
        let mut src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = src.as_slice_mut();
        got[ 0 ] = num!( 11 );
        got[ 1 ] = num!( 22 );
        let exp = $Va $( :: $Vb )* ::< T >::from2( num!( 11, 22 ) );
        a_id!( &src, &exp );
      }

      /* test.case = "into Canonical from Vector"; */
      {
        let src = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        let got = X2::< T >::from2( src );
        let exp = X2::< T >::from2( num!( 1, 2 ) );
        a_id!( got, exp );
        let got : X2< T > = src.into2();
        let exp = X2::< T >::from2( num!( 1, 2 ) );
        a_id!( got, exp );
      }

      /* test.case = "into Vector from Canonical"; */
      {
        let src = X2::< T >::from2( num!( 1, 2 ) );
        let got = $Va $( :: $Vb )* ::< T >::from2( src );
        let exp = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        a_id!( got, exp );
        let got : $Va $( :: $Vb )* < T > = src.into2();
        let exp = $Va $( :: $Vb )* ::< T >::from2( num!( 1, 2 ) );
        a_id!( got, exp );
      }

      // --

      /* test.case = "from / into tuple"; */
      {
        let src = ( num!( 1 ), num!( 2 ) );
        let got : $Va $( :: $Vb )*< T > = src.into2();
        let exp = $Va $( :: $Vb )*::< T >::make( num!( 1 ), num!( 2 ) );
        a_id!( got, exp );
        let got = $Va $( :: $Vb )*::< T >::from2( src );
        let exp = $Va $( :: $Vb )*::< T >::make( num!( 1 ), num!( 2 ) );
        a_id!( got, exp );
      }

      /* test.case = "from / into array"; */
      {
        let src = [ num!( 1 ), num!( 2 ) ];
        let got : $Va $( :: $Vb )*< T > = src.into2();
        let exp = $Va $( :: $Vb )*::< T >::make( num!( 1 ), num!( 2 ) );
        a_id!( got, exp );
        let got = $Va $( :: $Vb )*::< T >::from2( src );
        let exp = $Va $( :: $Vb )*::< T >::make( num!( 1 ), num!( 2 ) );
        a_id!( got, exp );
      }

      /* test.case = "from / into slice"; */
      {
        let _src = [ num!( 1 ), num!( 2 ) ];
        let src = &_src[ .. ];
        let got : $Va $( :: $Vb )*< T > = src.into2();
        let exp = $Va $( :: $Vb )*::< T >::make( num!( 1 ), num!( 2 ) );
        a_id!( got, exp );
        let got = $Va $( :: $Vb )*::< T >::from2( src );
        let exp = $Va $( :: $Vb )*::< T >::make( num!( 1 ), num!( 2 ) );
        a_id!( got, exp );
      }

      /* test.case = "debug format"; */
      {
        let src = $Va $( :: $Vb )*::< T >::from2( num!( 1, 2 ) );
        let got = format!( "{:?}", src );
        a_true!( got.len() > 0 );
      }

      // --

      $crate::macro_test_foreign_x2_number!( ( $Va $( :: $Vb )* , $_0, $_1 ) $( $( $tail ),* )? );
    }};

  }

  ///
  /// Tests for X2 conversion function. Names are implementation-specific. .
  ///


  macro_rules! macro_test_foreign_x2_as_specific
  {

    ( $Va : ident $( :: $Vb : ident )*, $Name : ident ; ) =>
    {
    };

    ( $Va : ident $( :: $Vb : ident )*, $Name : ident ; $Type : ident $(, $( $tail : ident ),* )? ) =>
    {

      /* test.case = "clone_as_nalgebra"; */
      {
        let src = X2::< T >::make( 1, 2 );
        let got = paste::paste!( src.[< clone_as_ $Name >]() );
        let exp = $Va $( :: $Vb )*::< T >::make( 1, 2 );
        a_id!( got, exp );
        a_true!( !mem_tools::mem::same_ptr( &got, &src ) );
      }

      /* test.case = "as_nalgebra"; */
      {
        let src = X2::< T >::make( 1, 2 );
        let got = paste::paste!( src.[< as_ $Name >]() );
        let exp = $Va $( :: $Vb )*::< T >::make( 1, 2 );
        a_id!( *got, exp );
        a_true!( mem_tools::mem::same_region( got, &src ) );
      }

      /* test.case = "as_nalgebra_mut"; */
      {
        let mut src = X2::< T >::make( 1, 2 );
        let got = paste::paste!( src.[< as_ $Name _mut>]() );
        let exp = $Va $( :: $Vb )*::< T >::make( 1, 2 );
        a_id!( *got, exp );
        got.assign( ( 11, 22 ) );
        let exp = X2::< T >::make( 11, 22  );
        a_id!( src, exp );
      }

      $crate::macro_foreign_x2::macro_test_foreign_x2_as_specific!( $Va $( :: $Vb )*, $Name ; $( $( $tail ),* )? );
    }

  }

  ///
  /// Tests for X2 conversion function as clone_as_foreign, as_foreign, as_foreign_mut .
  ///


  macro_rules! macro_test_foreign_x2_as_foreign
  {

    ( $Va : ident $( :: $Vb : ident )* ; ) =>
    {
    };

    ( $Va : ident $( :: $Vb : ident )* ; $Type : ident $(, $( $tail : ident ),* )? ) =>
    {

      /* test.case = "clone_as_foreign"; */
      {
        let src = X2::< T >::make( 1, 2 );
        let got = src.clone_as_foreign();
        let exp = $Va $( :: $Vb )*::< T >::make( 1, 2 );
        a_id!( got, exp );
        a_true!( !mem_tools::mem::same_ptr( &got, &src ) );
      }

      /* test.case = "as_foreign"; */
      {
        let src = X2::< T >::make( 1, 2 );
        let got = src.as_foreign();
        let exp = $Va $( :: $Vb )*::< T >::make( 1, 2 );
        a_id!( *got, exp );
        a_true!( mem_tools::mem::same_region( got, &src ) );
      }

      /* test.case = "as_foreign_mut"; */
      {
        let mut src = X2::< T >::make( 1, 2 );
        let got = src.as_foreign_mut();
        let exp = $Va $( :: $Vb )*::< T >::make( 1, 2 );
        a_id!( *got, exp );
        got.assign( ( 11, 22 ) );
        let exp = X2::< T >::make( 11, 22  );
        a_id!( src, exp );
      }

      $crate::macro_foreign_x2::macro_test_foreign_x2_as_foreign!( $Va $( :: $Vb )* ; $( $( $tail ),* )? );
    }
  }

  ///
  /// Tests for X2 conversion function. Names are implementation-specific. .
  ///


  macro_rules! macro_test_foreign_x2_operation_deref
  {

    ( $Va : ident $( :: $Vb : ident )* ; ) =>
    {
    };

    ( $Va : ident $( :: $Vb : ident )* ; $Type : ident $(, $( $tail : ident ),* )? ) =>
    {

      /* test.case = "neg"; */
      {
        let src1 = X2::< T >::make( 4, 3 );
        let got = - *src1;
        let exp = $Va $( :: $Vb )* ::< T >::make( -4, -3 );
        a_id!( got, exp );
      }

      /* test.case = "add"; */
      {
        let src1 = X2::< T >::make( 4, 3 );
        let src2 = X2::< T >::make( 2, 1 );
        let got = *src1 + *src2;
        let exp = $Va $( :: $Vb )* ::< T >::make( 6, 4 );
        a_id!( got, exp );
      }

      /* test.case = "sub"; */
      {
        let src1 = X2::< T >::make( 4, 3 );
        let src2 = X2::< T >::make( 1, 2 );
        let got = *src1 - *src2;
        let exp = $Va $( :: $Vb )* ::< T >::make( 3, 1 );
        a_id!( got, exp );
      }

      /* test.case = "dereferenced method"; */
      {
        let src1 = X2::< T >::make( 4, 3 );
        let got = src1.sum();
        let exp = 7;
        a_id!( got, exp );
      }

      $crate::macro_foreign_x2::macro_test_foreign_x2_operation_deref!( $Va $( :: $Vb )* ; $( $( $tail ),* )? );
    }
  }

  ///
  /// Template of a macro
  ///

  macro_rules! macro_test_foreign_x2_template
  {

    ( $Va : ident $( :: $Vb : ident )* ; ) =>
    {
    };

    ( $Va : ident $( :: $Vb : ident )* ; $Type : ident $(, $( $tail : ident ),* )? ) =>
    {

      /* test.case = "clone_as_foreign"; */
      {
      }

      $crate::macro_foreign_x2::macro_test_foreign_x2_template!( $Va $( :: $Vb )* ; $( $( $tail ),* )? );
    }
  }

  pub( crate ) use macro_test_foreign_x2_number_for_each;
  pub( crate ) use macro_test_foreign_x2_number;
  pub( crate ) use macro_test_foreign_x2_as_specific;
  pub( crate ) use macro_test_foreign_x2_as_foreign;
  pub( crate ) use macro_test_foreign_x2_operation_deref;
  pub( crate ) use macro_test_foreign_x2_template;

}

pub( crate ) use private::*;
