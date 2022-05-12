# Module :: wmath [![rust-status](https://github.com/Wandalen/wMath/actions/workflows/StandardRustPush.yml/badge.svg)](https://github.com/Wandalen/wMath/actions/workflows/StandardRustPush.yml) [![docs](https://img.shields.io/docsrs/wmath)](https://img.shields.io/docsrs/wmath) [![discord](https://img.shields.io/discord/872391416519737405)](https://img.shields.io/discord/872391416519737405) [![experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental)

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
cd wMath
cd sample/rust/wmath_trivial
cargo run
```
