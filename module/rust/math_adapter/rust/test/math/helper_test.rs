use wtest_basic::*;

//

macro_rules! print_type_name
{
  (
    $( $Args : tt )*
  ) =>
  {{
    println!( stringify!( $( $Args )* ) );
  }};
}

//

macro_rules! stringify_type_name
{
  (
    $X : tt
    $( $Args : tt )*
  ) =>
  {{
    $X.push_str( stringify!( $( $Args )* ) );
    $X.push_str( "_" );
  }};
}

//

fn _for_each_int_test()
{

  math_adapter::for_each_int!( print_type_name );
  math_adapter::for_each_int!( print_type_name where @POSTFIX . );

  let mut got = String::new();
  math_adapter::for_each_int!( stringify_type_name where @PREFIX got );
  let exp = "i8_i16_i32_i64_i128_u8_u16_u32_u64_u128_";
  assert_eq!( got, exp );

  let mut got = String::new();
  math_adapter::for_each_int!( stringify_type_name where @PREFIX got @POSTFIX . );
  let exp = "i8._i16._i32._i64._i128._u8._u16._u32._u64._u128._";
  assert_eq!( got, exp );

  let mut got = String::new();
  math_adapter::for_each_int!( stringify_type_name where @PREFIX got @POSTFIX( a b c ) );
  let exp = "i8(a b c)_i16(a b c)_i32(a b c)_i64(a b c)_i128(a b c)_u8(a b c)_u16(a b c)_u32(a b c)_u64(a b c)_u128(a b c)_";
  assert_eq!( got, exp );

}

//

fn _for_each_float_test()
{

  math_adapter::for_each_float!( print_type_name );

  let mut got = String::new();
  math_adapter::for_each_float!( stringify_type_name where @PREFIX got );
  let exp = "f32_f64_";
  assert_eq!( got, exp );

  let mut got = String::new();
  math_adapter::for_each_float!( stringify_type_name where @PREFIX got @POSTFIX . );
  let exp = "f32._f64._";
  assert_eq!( got, exp );

}

//

fn _for_each_number_test()
{

  math_adapter::for_each_number!( print_type_name );

  let mut got = String::new();
  math_adapter::for_each_number!( stringify_type_name where @PREFIX got );
  let exp = "i8_i16_i32_i64_i128_u8_u16_u32_u64_u128_f32_f64_";
  assert_eq!( got, exp );

  let mut got = String::new();
  math_adapter::for_each_number!( stringify_type_name where @PREFIX got @POSTFIX . );
  let exp = "i8._i16._i32._i64._i128._u8._u16._u32._u64._u128._f32._f64._";
  assert_eq!( got, exp );

}

//

fn _for_each_higher_order_without_parentheses_test()
{

  macro_rules! test_with_prefix
  {
    (
      $Got : tt
      $( $Arg : tt )*
    ) =>
    {
      math_adapter::for_each_float!( stringify_type_name where @PREFIX $Got @POSTFIX $( $Arg )* );
    }
  }

  // macro_rules! test_with_postfix
  // {
  //   (
  //     ( $( $Arg : tt )* )
  //     $Got : tt
  //   ) =>
  //   {
  //     math_adapter::for_each_float!( stringify_type_name where @PREFIX $Got @POSTFIX $( $Arg )* );
  //   }
  // }

  let mut got = String::new();
  math_adapter::for_each_float!( stringify_type_name where @PREFIX got @POSTFIX a );
  math_adapter::for_each_float!( stringify_type_name where @PREFIX got @POSTFIX b );
  math_adapter::for_each_float!( stringify_type_name where @PREFIX got @POSTFIX c );
  let exp = "f32 a_f64 a_f32 b_f64 b_f32 c_f64 c_";
  assert_eq!( got, exp );

  let mut got = String::new();
  math_adapter::for_each!( test_with_prefix where @PREFIX got @EACH a, b, c );
  let exp = "f32 a_f64 a_f32 b_f64 b_f32 c_f64 c_";
  assert_eq!( got, exp );

  // let mut got = String::new();
  // math_adapter::for_each!( test_with_postfix where @POSTFIX got @EACH ( a, b, c ) );
  // let exp = "f32 a_f64 a_f32 b_f64 b_f32 c_f64 c_";
  // assert_eq!( got, exp );

}

//

fn _braces_unwrap_test()
{
  static mut GOT : String = String::new();
  macro_rules! test_with
  {
    (
      $( $Arg : tt )*
    ) =>
    {{
      GOT += stringify!( $( $Arg )* );
      GOT += ";";
    }};
  }

  /* test.case( "sample1" ) */
  {
    let ( a, b, c ) = ( 1, 2, 3 );
    math_adapter::braces_unwrap!( dbg, { a, b, c } );
    // generates :
    // dbg!( a, b, c );
    math_adapter::braces_unwrap!( dbg, a, b, c );
    // generates :
    // dbg!( a, b, c );
  }

  /* test.case( "sample2" ) */
  {
    let ( prefix, a, b, c, postfix ) = ( "prefix", 1, 2, 3, "postfix" );
    math_adapter::braces_unwrap!
    (
      dbg where
      @PREFIX{ prefix, }
      @POSTFIX{ postfix }
      @SRC{ { a, b, c, } }
    );
    // generates :
    // dbg!( prefix, a, b, c, psotfix );
    math_adapter::braces_unwrap!
    (
      dbg where
      @PREFIX{ prefix, }
      @POSTFIX{ postfix }
      @SRC{ a, b, c, }
    );
    // generates :
    // dbg!( prefix, a, b, c, psotfix );
  }

  /* test.case( "function-style" ) */
  unsafe
  {
    GOT = "".to_string();
    math_adapter::braces_unwrap!( test_with, a, b, c );
    let exp = "a, b, c;";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!( test_with, { a, b, c } );
    let exp = "a, b, c;";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!( test_with, { { a, b, c } } );
    let exp = "{ a, b, c };";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!( test_with, ( a, b, c ) );
    let exp = "(a, b, c);";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!( test_with, ( ( a, b, c ) ) );
    let exp = "((a, b, c));";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!( test_with, [ a, b, c ] );
    let exp = "[a, b, c];";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!( test_with, [ [ a, b, c ] ] );
    let exp = "[[a, b, c]];";
    assert_eq!( GOT, exp );

  }

  /* test.case( "map-style" ) */
  unsafe
  {
    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @SRC{ a, b, c }
    );
    let exp = "a, b, c;";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @SRC{ { a, b, c } }
    );
    let exp = "a, b, c;";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @SRC{ { { a, b, c } } }
    );
    let exp = "{ a, b, c };";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @SRC{ ( a, b, c ) }
    );
    let exp = "(a, b, c);";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @SRC{ ( ( a, b, c ) ) }
    );
    let exp = "((a, b, c));";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @SRC{ [ a, b, c ] }
    );
    let exp = "[a, b, c];";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @SRC{ [ [ a, b, c ] ] }
    );
    let exp = "[[a, b, c]];";
    assert_eq!( GOT, exp );
  }

  /* test.case( "prefix and postfix" ) */
  unsafe
  {
    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ prefix }
      @POSTFIX{ postfix }
      @SRC{ a, b, c }
    );
    let exp = "prefix a, b, c postfix;";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ prefix }
      @POSTFIX{ postfix }
      @SRC{ { a, b, c } }
    );
    let exp = "prefix a, b, c postfix;";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ prefix }
      @POSTFIX{ postfix }
      @SRC{ { { a, b, c } } }
    );
    let exp = "prefix { a, b, c } postfix;";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ prefix }
      @POSTFIX{ postfix }
      @SRC{ ( a, b, c ) }
    );
    let exp = "prefix(a, b, c) postfix;";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ prefix }
      @POSTFIX{ postfix }
      @SRC{ ( ( a, b, c ) ) }
    );
    let exp = "prefix((a, b, c)) postfix;";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ prefix }
      @POSTFIX{ postfix }
      @SRC{ [ a, b, c ] }
    );
    let exp = "prefix [a, b, c] postfix;";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ prefix }
      @POSTFIX{ postfix }
      @SRC{ [ [ a, b, c ] ] }
    );
    let exp = "prefix [[a, b, c]] postfix;";
    assert_eq!( GOT, exp );

  }

  /* test.case( "prefix and postfix unwrapping" ) */
  unsafe
  {
    /* 0 */
    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ { prefix } }
      @POSTFIX{ { postfix } }
      @SRC{ { a, b, c } }
    );
    let exp = "prefix a, b, c postfix;";
    assert_eq!( GOT, exp );
    /* 1 */
    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ { prefix } }
      @POSTFIX{ { postfix } }
      @SRC{ { a, b, c } }
    );
    let exp = "prefix a, b, c postfix;";
    assert_eq!( GOT, exp );
    /* 2 */
    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ { prefix } }
      @POSTFIX{ { postfix } }
      @SRC{ { a, b, c } }
    );
    let exp = "prefix a, b, c postfix;";
    assert_eq!( GOT, exp );
    /* 3 */
    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ { prefix } }
      @POSTFIX{ { postfix } }
      @SRC{ { a, b, c } }
    );
    let exp = "prefix a, b, c postfix;";
    assert_eq!( GOT, exp );
    /* 4 */
    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ prefix }
      @POSTFIX{ { postfix } }
      @SRC{ { a, b, c } }
    );
    let exp = "prefix a, b, c postfix;";
    assert_eq!( GOT, exp );
    /* 5 */
    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ prefix }
      @POSTFIX{ { postfix } }
      @SRC{ a, b, c }
    );
    let exp = "prefix a, b, c postfix;";
    assert_eq!( GOT, exp );
    /* 6 */
    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ prefix }
      @POSTFIX{ postfix }
      @SRC{ { a, b, c } }
    );
    let exp = "prefix a, b, c postfix;";
    assert_eq!( GOT, exp );
    /* 7 */
    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ prefix }
      @POSTFIX{ postfix }
      @SRC{ a, b, c }
    );
    let exp = "prefix a, b, c postfix;";
    assert_eq!( GOT, exp );
  }

  /* test.case( "prefix" ) */
  unsafe
  {
    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ prefix }
      @SRC{ a, b, c }
    );
    let exp = "prefix a, b, c;";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ prefix }
      @SRC{ { a, b, c } }
    );
    let exp = "prefix a, b, c;";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ prefix }
      @SRC{ { { a, b, c } } }
    );
    let exp = "prefix { a, b, c };";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ prefix }
      @SRC{ ( a, b, c ) }
    );
    let exp = "prefix(a, b, c);";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ prefix }
      @SRC{ ( ( a, b, c ) ) }
    );
    let exp = "prefix((a, b, c));";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ prefix }
      @SRC{ [ a, b, c ] }
    );
    let exp = "prefix [a, b, c];";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ prefix }
      @SRC{ [ [ a, b, c ] ] }
    );
    let exp = "prefix [[a, b, c]];";
    assert_eq!( GOT, exp );

  }

  /* test.case( "prefix unwrapping" ) */
  unsafe
  {
    /* 0 */
    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ { prefix } }
      @SRC{ { a, b, c } }
    );
    let exp = "prefix a, b, c;";
    assert_eq!( GOT, exp );
    /* 1 */
    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ { prefix } }
      @SRC{ a, b, c }
    );
    let exp = "prefix a, b, c;";
    assert_eq!( GOT, exp );
    /* 2 */
    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ prefix }
      @SRC{ { a, b, c } }
    );
    let exp = "prefix a, b, c;";
    assert_eq!( GOT, exp );
    /* 3 */
    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @PREFIX{ prefix }
      @SRC{ a, b, c }
    );
    let exp = "prefix a, b, c;";
    assert_eq!( GOT, exp );
  }

  /* test.case( "postfix" ) */
  unsafe
  {
    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @POSTFIX{ postfix }
      @SRC{ a, b, c }
    );
    let exp = "a, b, c postfix;";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @POSTFIX{ postfix }
      @SRC{ { a, b, c } }
    );
    let exp = "a, b, c postfix;";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @POSTFIX{ postfix }
      @SRC{ { { a, b, c } } }
    );
    let exp = "{ a, b, c } postfix;";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @POSTFIX{ postfix }
      @SRC{ ( a, b, c ) }
    );
    let exp = "(a, b, c) postfix;";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @POSTFIX{ postfix }
      @SRC{ ( ( a, b, c ) ) }
    );
    let exp = "((a, b, c)) postfix;";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @POSTFIX{ postfix }
      @SRC{ [ a, b, c ] }
    );
    let exp = "[a, b, c] postfix;";
    assert_eq!( GOT, exp );

    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @POSTFIX{ postfix }
      @SRC{ [ [ a, b, c ] ] }
    );
    let exp = "[[a, b, c]] postfix;";
    assert_eq!( GOT, exp );

  }

  /* test.case( "postfix unwrapping" ) */
  unsafe
  {
    /* 0 */
    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @POSTFIX{ { postfix } }
      @SRC{ { a, b, c } }
    );
    let exp = "a, b, c postfix;";
    assert_eq!( GOT, exp );
    /* 1 */
    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @POSTFIX{ { postfix } }
      @SRC{ a, b, c }
    );
    let exp = "a, b, c postfix;";
    assert_eq!( GOT, exp );
    /* 2 */
    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @POSTFIX{ postfix }
      @SRC{ { a, b, c } }
    );
    let exp = "a, b, c postfix;";
    assert_eq!( GOT, exp );
    /* 3 */
    GOT = "".to_string();
    math_adapter::braces_unwrap!
    (
      test_with where
      @POSTFIX{ postfix }
      @SRC{ a, b, c }
    );
    let exp = "a, b, c postfix;";
    assert_eq!( GOT, exp );
  }

}

//

fn _for_each_test()
{

  macro_rules! test_with
  {
    (
      $( $Arg : tt )*
    ) =>
    {{
      GOT += stringify!( $( $Arg )* );
      GOT += "+";
    }};
  }

  static mut GOT : String = String::new();

  /* test.case( "sample function-style" ) */
  {
    math_adapter::for_each!( dbg, "a", "b", "c" );
  }

  // /* test.case( "sample map-style" ) */
  // {
  //   math_adapter::for_each!
  //   (
  //     dbg where
  //     @PREFIX { "prefix" + }
  //     @POSTFIX { + "postfix" }
  //     @EACH "a", "b", "c"
  //   );
  // }

  /* test.case( "function-style" ) */
  unsafe
  {
    GOT = "".to_string();
    math_adapter::for_each!( test_with, a, b, c );
    let exp = "a+b+c+";
    assert_eq!( GOT, exp );
  }

  /* test.case( "function-style, paths, unwrapping" ) */
  unsafe
  {
    GOT = "".to_string();
    math_adapter::for_each!( test_with, { std::collections::HashMap }, { std::collections::BTreeMap } );
    let exp = "std :: collections :: HashMap+std :: collections :: BTreeMap+";
    assert_eq!( GOT, exp );
  }

  /* test.case( "function-style, paths, parentheses" ) */
  unsafe
  {
    GOT = "".to_string();
    math_adapter::for_each!( test_with, ( std::collections::HashMap ), ( std::collections::BTreeMap ) );
    let exp = "(std :: collections :: HashMap)+(std :: collections :: BTreeMap)+";
    assert_eq!( GOT, exp );
  }

  // with parentheses

//   /* test.case( "with parentheses" ) */
//   unsafe
//   {
//     GOT = "".to_string();
//     math_adapter::for_each!( test_with where @EACH( a, b, c ) );
//     let exp = "(a)+(b)+(c)+";
//     assert_eq!( GOT, exp );
//   }
//
//   /* test.case( "with parentheses" ) + prefix */
//   unsafe
//   {
//     GOT = "".to_string();
//     math_adapter::for_each!( test_with where @PREFIX prefix @EACH( a, b, c ) );
//     let exp = "prefix(a)+prefix(b)+prefix(c)+";
//     assert_eq!( GOT, exp );
//   }
//
//   /* test.case( "with parentheses" ) + postfix */
//   unsafe
//   {
//     GOT = "".to_string();
//     math_adapter::for_each!( test_with where @POSTFIX postfix @EACH( a, b, c ) );
//     let exp = "(a) postfix+(b) postfix+(c) postfix+";
//     assert_eq!( GOT, exp );
//   }
//
//   /* test.case( "with parentheses" ) + prefix + postfix */
//   unsafe
//   {
//     GOT = "".to_string();
//     math_adapter::for_each!( test_with where @PREFIX prefix @POSTFIX postfix @EACH( a, b, c ) );
//     let exp = "prefix(a) postfix+prefix(b) postfix+prefix(c) postfix+";
//     assert_eq!( GOT, exp );
//   }

  // without parentheses

  /* test.case( "without parentheses" ) */
  unsafe
  {
    GOT = "".to_string();
    math_adapter::for_each!( test_with where @EACH a, b, c );
    let exp = "a+b+c+";
    assert_eq!( GOT, exp );
  }

  /* test.case( "without parentheses" ) + prefix */
  unsafe
  {
    GOT = "".to_string();
    math_adapter::for_each!( test_with where @PREFIX prefix @EACH a, b, c );
    let exp = "prefix a+prefix b+prefix c+";
    assert_eq!( GOT, exp );
  }

  /* test.case( "without parentheses" ) + postfix */
  unsafe
  {
    GOT = "".to_string();
    math_adapter::for_each!( test_with where @POSTFIX postfix @EACH a, b, c );
    let exp = "a postfix+b postfix+c postfix+";
    assert_eq!( GOT, exp );
  }

  /* test.case( "without parentheses" ) + prefix + postfix */
  unsafe
  {
    GOT = "".to_string();
    math_adapter::for_each!( test_with where @PREFIX prefix @POSTFIX postfix @EACH a, b, c );
    let exp = "prefix a postfix+prefix b postfix+prefix c postfix+";
    assert_eq!( GOT, exp );
  }

}

// //
//
// fn _for_each_samples_test()
// {
//
//   math_adapter::for_each!( dbg, "a", "b", "c" );
//
//   // xxx yyy
//   // math_adapter::for_each!
//   // (
//   //   dbg where
//   //   @PREFIX { "prefix" + }
//   //   @POSTFIX { + "postfix" }
//   //   @EACH "a", "b", "c"
//   // );
//
// }

//

test_suite!
{
  for_each_int_test,
  for_each_float_test,
  for_each_number_test,
  for_each_higher_order_without_parentheses_test,
  braces_unwrap_test,
  for_each_test,
  // for_each_samples_test,
}
