use test_tools::*;

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

tests_impls!
{
  fn for_each_int()
  {

    math_adapter::for_each_int!( print_type_name );
    math_adapter::for_each_int!( print_type_name where @Postfix . );

    let mut got = String::new();
    math_adapter::for_each_int!( stringify_type_name where @Prefix got );
    let exp = "isize_i8_i16_i32_i64_i128_usize_u8_u16_u32_u64_u128_";
    a_id!( got, exp );

    let mut got = String::new();
    math_adapter::for_each_int!( stringify_type_name where @Prefix got @Postfix . );
    let exp = "isize._i8._i16._i32._i64._i128._usize._u8._u16._u32._u64._u128._";
    a_id!( got, exp );

    let mut got = String::new();
    math_adapter::for_each_int!( stringify_type_name where @Prefix got @Postfix( a b c ) );
    let exp = "isize(a b c)_i8(a b c)_i16(a b c)_i32(a b c)_i64(a b c)_i128(a b c)_usize(a b c)_u8(a b c)_u16(a b c)_u32(a b c)_u64(a b c)_u128(a b c)_";
    a_id!( got, exp );

  }

  //

  fn for_each_float()
  {

    math_adapter::for_each_float!( print_type_name );

    let mut got = String::new();
    math_adapter::for_each_float!( stringify_type_name where @Prefix got );
    let exp = "f32_f64_";
    a_id!( got, exp );

    let mut got = String::new();
    math_adapter::for_each_float!( stringify_type_name where @Prefix got @Postfix . );
    let exp = "f32._f64._";
    a_id!( got, exp );

  }

  //

  fn for_each_number()
  {

    math_adapter::for_each_number!( print_type_name );

    let mut got = String::new();
    math_adapter::for_each_number!( stringify_type_name where @Prefix got );
    let exp = "isize_i8_i16_i32_i64_i128_usize_u8_u16_u32_u64_u128_f32_f64_";
    a_id!( got, exp );

    let mut got = String::new();
    math_adapter::for_each_number!( stringify_type_name where @Prefix got @Postfix . );
    let exp = "isize._i8._i16._i32._i64._i128._usize._u8._u16._u32._u64._u128._f32._f64._";
    a_id!( got, exp );

  }

  //

  fn for_each_higher_order_without_parentheses()
  {

    macro_rules! test_with_prefix
    {
      (
        $Got : tt
        $( $Arg : tt )*
      ) =>
      {
        math_adapter::for_each_float!( stringify_type_name where @Prefix $Got @Postfix $( $Arg )* );
      }
    }

    // macro_rules! test_with_postfix
    // {
    //   (
    //     ( $( $Arg : tt )* )
    //     $Got : tt
    //   ) =>
    //   {
    //     math_adapter::for_each_float!( stringify_type_name where @Prefix $Got @Postfix $( $Arg )* );
    //   }
    // }

    let mut got = String::new();
    math_adapter::for_each_float!( stringify_type_name where @Prefix got @Postfix a );
    math_adapter::for_each_float!( stringify_type_name where @Prefix got @Postfix b );
    math_adapter::for_each_float!( stringify_type_name where @Prefix got @Postfix c );
    let exp = "f32 a_f64 a_f32 b_f64 b_f32 c_f64 c_";
    a_id!( got, exp );

    let mut got = String::new();
    math_adapter::for_each!( test_with_prefix where @Prefix got @Each a b c );
    let exp = "f32 a_f64 a_f32 b_f64 b_f32 c_f64 c_";
    a_id!( got, exp );

    // let mut got = String::new();
    // math_adapter::for_each!( test_with_postfix where @Postfix got @Each ( a, b, c ) );
    // let exp = "f32 a_f64 a_f32 b_f64 b_f32 c_f64 c_";
    // a_id!( got, exp );

  }
}

//

tests_index!
{
  for_each_int,
  for_each_float,
  for_each_number,
  for_each_higher_order_without_parentheses,
}
