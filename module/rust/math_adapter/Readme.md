# module::math_adapter

Collection of math adapters to decouple your application from math libraries' implementations and to provide both inter-libraries compatibility and affordable exchangeability.

### Sample

```rust
use math_adapter::prelude::*;
use math_adapter::X2;

fn main()
{

  /* vector of length 2 and its components */
  let src1 = X2::make( 1, 3 );
  assert_eq!( src1.x(), 1 );
  assert_eq!( src1.y(), 3 );

  /* using operators  */
  #[ cfg( any( feature = "cgmath_ops", feature = "nalgebra_ops", feature = "default_ops" ) ) ]
  {
    let src1 = X2::make( 1, 2 );
    let src2 = X2::make( 3, 4 );
    let got = src1 + src2;
    let exp = X2::make( 4, 6 );
    assert_eq!( got, exp );
  }

}

/* xxx : adjust sample */

```

### To add to your project

```sh
cargo add math_adapter
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd module/math_adapter/sample/rust/math_adapter_trivial
cargo run
```
