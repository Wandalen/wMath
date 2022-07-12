<!-- {{# generate.module_header{} #}} -->

# Module :: math_adapter [![experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wMath/actions/workflows/ModuleMathAdapterPush.yml/badge.svg)](https://github.com/Wandalen/wMath/actions/workflows/ModuleMathAdapterPush.yml) [![docs.rs](https://img.shields.io/docsrs/math_adapter?color=e3e8f0&logo=docs.rs)](https://docs.rs/math_adapter) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fmath_adapter_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20math_adapter_trivial_sample/https://github.com/Wandalen/wMath) [![discord](https://img.shields.io/discord/872391416519737405?color=e3e8f0&logo=discord&logoColor=e3e8f0)](https://discord.gg/JwTG6d2b)

Collection of math adapters to decouple your application from math libraries implementations and to provide both inter-libraries compatibility and affordable exchangeability.

Been decoupled from concrete implementation of math library is useful. It makes possible to interchange them and leverage interoperability between them.

# Choosing back-end

Currently, as back-end you may choose either `cgmath` or `nalgebra` and both math libraries are integrated into the crate. Despite that, by default, `math_adapter` does not include either of them and does not increase the size of the executable by unnecessary dependencies. To chose back-end use features:

- `cgmath` - use `cgmath` as backend
- `cgmath_ops` - use `cgmath` as backend and make adapters to derefer to its functions and operators
- `nalgebra` - use `nalgebra` as backend
- `nalgebra_ops` - use `nalgebra` as backend and make adapters to derefer to its functions and operators

Several math libraries can be used, but functions and operators of only one should be used implicitly. Even if no `*_ops` feature is selected, it is always an option to use functions and operators of math library of choice explicitly.

To make explicit conversion, use methods `clone_as_cgmath()`, `clone_as_nalgebra()`. To make explicit conversion into selected with feature `*_ops` use `clone_as_foreign()`. In this case, dereferencing works as reinterpretation. Either adapter or foreign math object may be converted/reinterpreted into analog.

In the future, each back-end is going to reside in an individual crate.

For example, to use `nalgebra` with operators and its function include `math_adapter` like that:

```toml
math_adapter = { version = "*", features = [ "nalgebra_ops" ] }
```

# Math objects

Each math libraries define its own versions of math objects. In most cases they have the same layout, but even if so compiler treat them as different. `math_adapter` provides such math objects too, as well as means to either convert or reinterpret from foreign analogs. Among such are:

- Vectors, they are called X1< T >, X2< T >, X3< T >, X4< T > ... Xn< T >.
- Matrices, they are called MatX1< T >, MatX2< T >, MatX3< T > ... MatXn< T >.
- Quaternion, it is called Quat< T >.
- Euler's angles, it is called Euler< T >.
- Decomposed transformation, it is called TransformationDecomposed< T >

<!-- qqq : add readme for each sample with short explanation. make sure code frome sample run during test -->

### Conversion vs reinterpretation

To apply functions from another library you may either convert or reinterpret math object. What is difference?

Reinterpretation take place during compile-time. Reinterpretation is possible if layout ( size, alignment, padding and order ) of two similar structures are the same. For example, structures `cgmath::Vector2` and `nalgebra::Vector2` are different structures, but them both have exactly same layout. That's why it's safe to reinterpret one to another and vise-versa. Reinterpretation says compiler to use one data structure as if it was another. It has zero run-time and compile-time cost. Such methods as `as_foreign()`, `as_foreign_mut()`, `as_cgmath()`, `as_cgmath_mut()`, `as_nalgebra()`, `as_nalgebra_mut()` reinterpret math objects.

Conversion is different. Conversion says to rebuild a new instance of structure from components of another. It has non-zero run-time and compile-time cost. Although it is often optimized into reinterpretation by compiler. Using conversion instead of reinterpretation is safer. Thing is reinterpretation is safe based on several assumption about layout, which may be changed by either an author of a math library or by authors of the compiler. In theory! On practice it is unlikely. Even more most math libraries define objects with `#[ repr( C ) ]`, what [restricts layout](https://doc.rust-lang.org/nomicon/other-reprs.html#reprc) of such structures and protects them from changes in the future.

Every structure could be converted into another semantically similar structure even with different layout, but reinterpretation is possible only in case of the same layout. Because of that several traits are implemented.

- `*NominalInterface` - interface, exposing function to convert and to access elements.
- `*BasicInterface` - interface, extending `X2NominalInterface` and exposing functions to make a new instance of such.
- `*CanonicalInterface` - interface, extending `X2BasicInterface` and exposing functions of reinterpretation.

Relation: `Canonical > Basic > Nominal`.

### Sample :: elements

Number of elements of a vector is coded in name of type, `X2` for vector of length `2`, `X3` for vector of length 3 and so on. Each structure implements constructors `make()`, `make_nan()`, and `make_default()` to construct a new instance of the type. To get access to elements use either methods `x()`, `y()`, `z()` or `_0()`, `_1()`, `_2()`.

<!-- {{# generate.module_sample{} #}} -->

```rust
#[ cfg( feature = "use_std" ) ]
{
  use math_adapter::prelude::*;
  use math_adapter::X2;

  // vector of length 2 and its elements
  let src1 = X2::make( 1, 3 );
  assert_eq!( src1.x(), 1 );
  assert_eq!( src1.y(), 3 );
  assert_eq!( src1._0(), 1 );
  assert_eq!( src1._1(), 3 );
}
```

### Sample :: operators

Select a feature `*_ops` to reuse operators and function of math lib of choice.

<!-- {{# generate.module_sample{} #}} -->

```rust
#[ cfg( feature = "use_std" ) ]
{
  use math_adapter::prelude::*;
  use math_adapter::X2;

  // if back-end math lib is chosen then operators and functions are available
  #[ cfg( feature = "cgmath_ops" ) ]
  {
    let src1 = X2::make( 1, 2 );
    let src2 = X2::make( 3, 4 );
    let got = src1 + src2;
    let exp = X2::make( 4, 6 );
    assert_eq!( got, exp );
    println!( "src1 + src2 : {:?}", got );
  }

  // enable feature *_ops to get access to functions
  #[ cfg( feature = "cgmath_ops" ) ]
  {
    let src = X2::make( 1, 2 );
    assert_eq!( src.sum(), 3 ); /* sum comes from `cgmath` */
    println!( "src.sum() : {:?}", src.sum() );
  }
}
```

### Sample :: interoperability

Feature `*_ops` means to request to use operators and function of math lib of choice. But instead of choosing a single back-end math lib, you may use several. Use methods `as_*_clone()`, `as_*`, `as_*_mut` to either convert or reinterpret original math object into analog of such of chosen back-end. You don't have to use the same back-end in every call, you may choose which math lib for a specific call and combine the best of each math lib.

<!-- {{# generate.module_sample{} #}} -->

```rust
#[ cfg( feature = "use_std" ) ]
{
  use math_adapter::prelude::*;
  use math_adapter::X2;

  // ! compile-time error, because if no `*_ops` feature was chosen
  // {
  //   let src = X2::make( 1, 3 ); /* make a canonical 2D vector */
  //   println!( "src.sum() : {:?}", src.sum() ); /* use `sum()` of chosen math lib back-end */
  // }

  // enable feature *_ops should be enabled to get access to functions
  #[ cfg( all( feature = "cgmath", feature = "nalgebra" ) ) ]
  {
    let src = X2::make( 1, 3 ); /* make a canonical 2D vector */
    println!( "src.as_cgmath().sum() : {:?}", src.as_cgmath().sum() ); /* use `sum()` of `cgmath` */
    println!( "src.as_nalgebra().sum() : {:?}", src.as_nalgebra().sum() ); /* use `sum()` of `nalgebra` */
  }

  // you can convert / reinterpret any vector.
  // for example you can create `cgmath::Vector2`, but apply a function of `nalgebra::Vector2`
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
cd wMath
cd module/math_adapter/sample/rust/math_adapter_trivial_sample
cargo run
```
