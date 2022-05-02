# module::wmath

Math library aggregating several math modules.

### Sample

```rust
use wmath::adapter::prelude::*;

fn main()
{
  let x2_original = wmath::X2::< u8 >( 1, 3 );
  println!( "{:?}", x2_original );
  /* log : X2(1, 3) */
  let x2_to_array = x2_original.clone_as_array();
  println!( "{:?}", x2_to_array );
  /* log : [1, 3] */
}
```

### To add to your project

```sh
cargo add wmath
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wMath
cd wTools
cd sample/rust/wmath_trivial
cargo run
```
