# module::math_adapter

Collection of math adapters to decouple your application from math libraries' implementations and to provide both inter-libraries compatibility and affordable exchangeability.

<!-- xxx : mention adapter pattern -->
<!-- xxx : describe process of choosing back-end math lib -->
<!-- xxx : explain nominal/basic/canonical difference -->
<!-- qqq : add readme for each sample with short explanation. make sure code frome sample run during test -->
<!-- xxx : define math object -->

### Conversion vs reinterpretation

To apply functions from another library you may either convert or reinterpret math object. What is difference?

Reinterpretation take place during compile-time. Reinterpretation is possible if layout( size, alignment, padding and order ) of two similar structures are the same. For example structures `cgmath::Vector2` and `nalgebra::Vector2` are different structures, but them both have exactly same layout. That's why it's safe to reinterpret one to another and vise-versa. Reinterpretation says compiler to use one data structure as if it was another. It has zero run-time and compile-time cost. Such methods as `as_native()`, `as_native_mut()`, `as_cgmath()`, `as_cgmath_mut()`, `as_nalgebra()`, `as_nalgebra_mut()` reintepret math objects.

Conversion is different. Conversion says to rebuild a new instance of structure from components of another. It has non-zero run-time and compile-time cost. Although it is often optimized into reinterpretation by compiler. Also argument to use conversion instead of reinterpretation is safety. Thing is reinterpretation is safe based on several assumption about layout, which may be changed by either an author of a math library or by authors of the compiler. In theory! On practice it is unlikely. Even more most math objects are declared with `#[ repr( C ) ]`, what [restricts layout](https://doc.rust-lang.org/nomicon/other-reprs.html#reprc) of such structure and protects it from changes in the future.

Every structure could be converted into another semantically similar structure even with different layout, but reinterpreation is possible only in case of the same layour. Because of that severa traits are implemented.

- `*NominalInterface` - interface exposing function to convert and to access elements.
- `*BasicInterface` - interface extending `X2NominalInterface` and exposing functions to make a new instance of such.
- `*CanonicalInterface` - interface extending `X2BasicInterface` and exposing functions of reinterpretation.

Relation: `Canonical > Basic > Nominal`.


### Sample : elements

Number of elements of a vector is coded in name of type, `X2` for vector of length `2`, `X3` for vector of length 3 and so on. Each structure implements constructors `make()`, `make_nan()`, and `make_default()` to construct a new instance of the type. To get access to elements use either methods `x()`, `y()`, `z()` or `_0()`, `_1()`, `_2()`.

```rust
use math_adapter::prelude::*;
use math_adapter::X2;

fn main()
{

  /* vector of length 2 and its elements */
  let src1 = X2::make( 1, 3 );
  assert_eq!( src1.x(), 1 );
  assert_eq!( src1.y(), 3 );
  assert_eq!( src1._0(), 1 );
  assert_eq!( src1._1(), 3 );

}

```

### Sample : operators

Select a feature `*_ops` to reuse operators and function of math lib of choice.

```rust
use math_adapter::prelude::*;
use math_adapter::X2;

fn main()
{

  /* if back-end math lib is chosen then operators and functions are available */
  #[ cfg( feature = "cgmath_ops" ) ]
  {
    let src1 = X2::make( 1, 2 );
    let src2 = X2::make( 3, 4 );
    let got = src1 + src2;
    let exp = X2::make( 4, 6 );
    assert_eq!( got, exp );
    println!( "src1 + src2 : {:?}", got );
  }

  /* enable feature *_ops to get access to functions */
  #[ cfg( feature = "cgmath_ops" ) ]
  {
    let src = X2::make( 1, 2 );
    assert_eq!( src.sum(), 3 ); /* sum comes from `cgmath` */
    println!( "src.sum() : {:?}", src.sum() );
  }

}
```

### Sample : interoperability

Feature `*_ops` means to request to use operators and function of math lib of choice. But instead of choosing a single back-end math lib, you may use several. Use methods `as_*_clone()`, `as_*`, `as_*_mut` to either convert or reinterpret original math object into analog of such of chosen back-end. You don't have to use the same back-end in every call, you may choose which math lib for a specific call and combine the best of each math lib.

```rust
use math_adapter::prelude::*;
use math_adapter::X2;

fn main()
{

  /* ! compile time error, because if no `*_ops` feature was chosen */
  // {
  //   let src = X2::make( 1, 3 ); /* make a canonical 2D vector */
  //   println!( "src.sum() : {:?}", src.sum() ); /* use `sum()` of chosen math lib back-end */
  // }

  /* enable feature *_ops should be enabled to get access to functions */
  #[ cfg( all( feature = "cgmath", feature = "nalgebra" ) ) ]
  {
    let src = X2::make( 1, 3 ); /* make a canonical 2D vector */
    println!( "src.as_cgmath().sum() : {:?}", src.as_cgmath().sum() ); /* use `sum()` of `cgmath` */
    println!( "src.as_nalgebra().sum() : {:?}", src.as_nalgebra().sum() ); /* use `sum()` of `nalgebra` */
  }

  /* you can convert / reinterpret any vector. for example you can create `cgmath::Vector2`, but apply a function of `nalgebra::Vector2` */
  #[ cfg( all( feature = "cgmath", feature = "nalgebra" ) ) ]
  {
    let src = math_adapter::cgmath::Vector2::< i32 >::make( 1, 3 ); /* make a `cgmath` 2D vector */
    println!( "src.as_nalgebra().sum() : {:?}", src.as_nalgebra().sum() ); /* use `sum()` of `nalgebra` */
  }

}
```

### To add to your project

```sh
cargo add math_adapter
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wMath
cd wTools
cd module/math_adapter/sample/rust/math_adapter_trivial
cargo run
```

<!-- xxx : implement `make_nan()`, and `make_default()` -->
<!-- xxx : implement print trait -->

