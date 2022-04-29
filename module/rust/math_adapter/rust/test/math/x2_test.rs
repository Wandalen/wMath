use wtest_basic::*;
use core::mem::size_of;
use math_adapter::prelude::*;
use num_traits::cast::cast;
use crate::tools::*;

///
/// One test should be ordinary to exclude possibility of problems with macro.
///

fn _basic()
{
  type T = i8;

  /* test.case = "size"; */
  {
    assert_eq!( size_of::< math_adapter::x2::< T > >(), size_of::< ( T, T ) >() );
    assert_eq!( size_of::< math_adapter::x2::< T > >(), size_of::< [ T ; 2 ] >() );
    assert_eq!( size_of::< math_adapter::x2::< T > >(), 2 );
  }

  /* test.case = "value of elements"; */
  {
    let got = math_adapter::x2::< i8 >( 1, 2 );
    assert_eq!( got.0, 1 );
    assert_eq!( got.1, 2 );
    assert_eq!( got._0(), 1 );
    assert_eq!( got._1(), 2 );
  }

  /* test.case = "operator add"; */
  {
    let src1 = math_adapter::x2::< i8 >( 1, 2 );
    let src2 = math_adapter::x2::< i8 >( 2, 3 );
    let got = src1 + src2;
    let exp = math_adapter::x2::< i8 >( 3, 5 );
    assert_eq!( got, exp );
  }

}

//

#[ cfg( all( feature = "winit", feature = "cgmath" ) ) ]
#[ test ]
fn cgmath_winit_interoperability_test()
{

  /* test.case = "use cgmath vectors for operations on winit vectors"; */
  {
    use math_adapter::AsCgmathInterface;
    let src1 = winit::dpi::PhysicalSize::< i8 >::make( 3, 2 );
    let src2 = winit::dpi::PhysicalSize::< i8 >::make( 0, 1 );
    let got = src1.as_cgmath() - src2.as_cgmath();
    let exp = cgmath::Vector2::< i8 >::make( 3, 1 );
    assert_eq!( got, exp );
  }

}

//

macro_rules! test_for
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
        assert_eq!( size_of::< math_adapter::x2::< T > >(), size_of::< ( T, T ) >() );
        assert_eq!( size_of::< math_adapter::x2::< T > >(), size_of::< [ T ; 2 ] >() );
      }

      /* test.case = "value of elements"; */
      {
        let got = math_adapter::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        assert_eq!( got.0, cast::< _, T >( 1 ).unwrap() );
        assert_eq!( got.1, cast::< _, T >( 2 ).unwrap() );
        assert_eq!( got._0(), cast::< _, T >( 1 ).unwrap() );
        assert_eq!( got._1(), cast::< _, T >( 2 ).unwrap() );
        assert_eq!( got.x(), cast::< _, T >( 1 ).unwrap() );
        assert_eq!( got.y(), cast::< _, T >( 2 ).unwrap() );
        assert_eq!( *got._0_ref(), cast::< _, T >( 1 ).unwrap() );
        assert_eq!( *got._1_ref(), cast::< _, T >( 2 ).unwrap() );
      }

      /* test.case = "set value of elements"; */
      {
        let mut got = math_adapter::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
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
        let got = math_adapter::x2::< T >::make( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let exp = math_adapter::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        assert_eq!( got, exp );
      }

      /* test.case = "clone_as_tuple"; */
      {
        let src = math_adapter::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.clone_as_tuple();
        let exp : ( T , T ) = ( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        assert_eq!( got, exp );
        assert!( !mem_same_ptrs( &got, &src ) ); /* qqq : discuss */
      }

      /* test.case = "clone_as_array"; */
      {
        let src = math_adapter::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.clone_as_array();
        let exp : [ T ; 2 ] = [ cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() ];
        assert_eq!( got, exp );
        assert!( !mem_same_ptrs( &got, &src ) );
      }

      /* test.case = "clone_as_canonical"; */
      {
        let src = math_adapter::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.clone_as_canonical();
        let exp = math_adapter::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        assert_eq!( got, exp );
        assert!( !mem_same_ptrs( &got, &src ) );
      }

      // --

      /* test.case = "as_tuple"; */
      {
        let src = math_adapter::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_tuple();
        let exp : ( T , T ) = ( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        assert_eq!( got, &exp );
        assert!( mem_same( got, &src ) ); /* qqq : discuss */
      }

      /* test.case = "as_array"; */
      {
        let src = math_adapter::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_array();
        let exp : [ T ; 2 ] = [ cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() ];
        assert_eq!( got, &exp );
        assert!( mem_same( got, &src ) );
      }

      /* test.case = "as_canonical"; */
      {
        let src = math_adapter::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_canonical();
        let exp = math_adapter::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        assert_eq!( got, &exp );
        assert!( mem_same( got, &src ) );
      }

      /* test.case = "as_slice"; */
      {
        let src = math_adapter::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_slice();
        let exp = &[ cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() ][ .. ];
        assert_eq!( got, exp );
      }

      // --

      /* test.case = "as_tuple_mut"; */
      {
        let mut src = math_adapter::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_tuple_mut();
        got.0 = cast::< _, T >( 11 ).unwrap();
        got.1 = cast::< _, T >( 22 ).unwrap();
        let exp = math_adapter::x2::< T >( cast::< _, T >( 11 ).unwrap(), cast::< _, T >( 22 ).unwrap() );
        assert_eq!( &src, &exp );
      }

      /* test.case = "as_array"; */
      {
        let mut src = math_adapter::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_array_mut();
        got[ 0 ] = cast::< _, T >( 11 ).unwrap();
        got[ 1 ] = cast::< _, T >( 22 ).unwrap();
        let exp = math_adapter::x2::< T >( cast::< _, T >( 11 ).unwrap(), cast::< _, T >( 22 ).unwrap() );
        assert_eq!( &src, &exp );
      }

      /* test.case = "as_canonical"; */
      {
        let mut src = math_adapter::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_canonical_mut();
        *got._0_mut() = cast::< _, T >( 11 ).unwrap();
        *got._1_mut() = cast::< _, T >( 22 ).unwrap();
        let exp = math_adapter::x2::< T >( cast::< _, T >( 11 ).unwrap(), cast::< _, T >( 22 ).unwrap() );
        assert_eq!( &src, &exp );
      }

      /* test.case = "as_slice"; */
      {
        let mut src = math_adapter::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_slice_mut();
        got[ 0 ] = cast::< _, T >( 11 ).unwrap();
        got[ 1 ] = cast::< _, T >( 22 ).unwrap();
        let exp = math_adapter::x2::< T >( cast::< _, T >( 11 ).unwrap(), cast::< _, T >( 22 ).unwrap() );
        assert_eq!( &src, &exp );
      }

      // --

      /* test.case = "operator add"; */
      {
        let src1 = math_adapter::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let src2 = math_adapter::x2::< T >( cast::< _, T >( 2 ).unwrap(), cast::< _, T >( 3 ).unwrap() );
        let got = src1 + src2;
        let exp = math_adapter::x2::< T >( cast::< _, T >( 3 ).unwrap(), cast::< _, T >( 5 ).unwrap() );
        assert_eq!( got, exp );

      }

    }

    test_for!( $( $( $tail ),* )? );
  };

}

fn _canonical_test()
{
  test_for!( i8, i16, i32, i64, i128 );
  test_for!( u8, u16, u32, u64, u128 );
  test_for!( f32, f64 );
}

//

test_suite!
{
  basic,
  canonical_test,
}
