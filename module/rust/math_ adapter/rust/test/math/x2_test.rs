// #![ feature( concat_idents ) ]

use wtest_basic::*;
use core::mem::size_of;
use wmath::prelude::*;
// use num_traits::cast::AsPrimitive;
use num_traits::cast::cast;
// use paste::paste;

///
/// Are two pointers points on the same region.
///

// pub fn mem_same_ptrs< T1 : ?Sized, T2 : ?Sized >( src1 : &T1, src2 : &T2 ) -> bool
pub fn mem_same_ptrs< T1, T2 >( src1 : &T1, src2 : &T2 ) -> bool
{
  unsafe
  {
    std::mem::transmute::< *const T1, *const () >( src1 as *const T1 )
    ==
    std::mem::transmute::< *const T2, *const () >( src2 as *const T2 )
    // std::mem::transmute::< *const T1, *const T2 >( src1 as *const T1 ) == src2 as *const T2
  }
}

///
/// Are two pointers points on the same region.
///

pub fn mem_same_size< T1, T2 >( _src1 : &T1, _src2 : &T2 ) -> bool
{
  core::mem::size_of::< T1 >() == core::mem::size_of::< T2 >()
}

///
/// Are two pointers points on the same region.
///

pub fn mem_same< T1, T2 >( src1 : &T1, src2 : &T2 ) -> bool
{
  mem_same_ptrs( src1, src2 ) && mem_same_size( src1, src2 )
}

///
/// One test should be ordinary to exclude possibility of problems with macro.
///

fn _basic()
{
  type T = i8;

  assert_eq!( size_of::< wmath::x2::< T > >(), size_of::< ( T, T ) >() );
  assert_eq!( size_of::< wmath::x2::< T > >(), size_of::< [ T ; 2 ] >() );
  assert_eq!( size_of::< wmath::x2::< T > >(), 2 );

  let got = wmath::x2::< i8 >( 1, 2 );
  assert_eq!( got.0, 1 );
  assert_eq!( got.1, 2 );

  let src1 = wmath::x2::< i8 >( 1, 2 );
  let src2 = wmath::x2::< i8 >( 2, 3 );
  let got = src1 + src2;
  let exp = wmath::x2::< i8 >( 3, 5 );
  assert_eq!( got, exp );

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
        assert_eq!( size_of::< wmath::x2::< T > >(), size_of::< ( T, T ) >() );
        assert_eq!( size_of::< wmath::x2::< T > >(), size_of::< [ T ; 2 ] >() );
      }

      /* test.case = "value of elements"; */
      {
        let got = wmath::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
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
        let mut got = wmath::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
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
        let got = wmath::x2::< T >::make( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let exp = wmath::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        assert_eq!( got, exp );
      }

      /* test.case = "clone_as_tuple"; */
      {
        let src = wmath::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.clone_as_tuple();
        let exp : ( T , T ) = ( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        assert_eq!( got, exp );
        assert!( !mem_same_ptrs( &got, &src ) ); /* qqq : discuss */
      }

      /* test.case = "clone_as_array"; */
      {
        let src = wmath::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.clone_as_array();
        let exp : [ T ; 2 ] = [ cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() ];
        assert_eq!( got, exp );
        assert!( !mem_same_ptrs( &got, &src ) );
      }

      /* test.case = "clone_as_canonical"; */
      {
        let src = wmath::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.clone_as_canonical();
        let exp = wmath::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        assert_eq!( got, exp );
        assert!( !mem_same_ptrs( &got, &src ) );
      }

      // --

      /* test.case = "as_tuple"; */
      {
        let src = wmath::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_tuple();
        let exp : ( T , T ) = ( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        assert_eq!( got, &exp );
        assert!( mem_same( got, &src ) ); /* qqq : discuss */
      }

      /* test.case = "as_array"; */
      {
        let src = wmath::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_array();
        let exp : [ T ; 2 ] = [ cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() ];
        assert_eq!( got, &exp );
        assert!( mem_same( got, &src ) );
      }

      /* test.case = "as_canonical"; */
      {
        let src = wmath::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_canonical();
        let exp = wmath::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        assert_eq!( got, &exp );
        assert!( mem_same( got, &src ) );
      }

      /* test.case = "as_slice"; */
      {
        let src = wmath::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_slice();
        let exp = &[ cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() ][ .. ];
        assert_eq!( got, exp );
      }

      // --

      /* test.case = "as_tuple_mut"; */
      {
        let mut src = wmath::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_tuple_mut();
        got.0 = cast::< _, T >( 11 ).unwrap();
        got.1 = cast::< _, T >( 22 ).unwrap();
        let exp = wmath::x2::< T >( cast::< _, T >( 11 ).unwrap(), cast::< _, T >( 22 ).unwrap() );
        assert_eq!( &src, &exp );
      }

      /* test.case = "as_array"; */
      {
        let mut src = wmath::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_array_mut();
        got[ 0 ] = cast::< _, T >( 11 ).unwrap();
        got[ 1 ] = cast::< _, T >( 22 ).unwrap();
        let exp = wmath::x2::< T >( cast::< _, T >( 11 ).unwrap(), cast::< _, T >( 22 ).unwrap() );
        assert_eq!( &src, &exp );
      }

      /* test.case = "as_canonical"; */
      {
        let mut src = wmath::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_canonical_mut();
        *got._0_mut() = cast::< _, T >( 11 ).unwrap();
        *got._1_mut() = cast::< _, T >( 22 ).unwrap();
        let exp = wmath::x2::< T >( cast::< _, T >( 11 ).unwrap(), cast::< _, T >( 22 ).unwrap() );
        assert_eq!( &src, &exp );
      }

      /* test.case = "as_slice"; */
      {
        let mut src = wmath::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let got = src.as_slice_mut();
        got[ 0 ] = cast::< _, T >( 11 ).unwrap();
        got[ 1 ] = cast::< _, T >( 22 ).unwrap();
        let exp = wmath::x2::< T >( cast::< _, T >( 11 ).unwrap(), cast::< _, T >( 22 ).unwrap() );
        assert_eq!( &src, &exp );
      }

      // --

      /* test.case = "operator add"; */
      {
        let src1 = wmath::x2::< T >( cast::< _, T >( 1 ).unwrap(), cast::< _, T >( 2 ).unwrap() );
        let src2 = wmath::x2::< T >( cast::< _, T >( 2 ).unwrap(), cast::< _, T >( 3 ).unwrap() );
        let got = src1 + src2;
        let exp = wmath::x2::< T >( cast::< _, T >( 3 ).unwrap(), cast::< _, T >( 5 ).unwrap() );
        assert_eq!( got, exp );

      }

    }

    test_for!( $( $( $tail ),* )? );
  };

}

fn _all_primitives_test()
{
  test_for!( i8 );
  test_for!( i8, i16, i32, i64, i128 );
  test_for!( u8, u16, u32, u64, u128 );
  test_for!( f32, f64 );
}

//

test_suite!
{
  basic,
  all_primitives_test,
}
