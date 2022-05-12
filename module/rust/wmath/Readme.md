# Module :: wmath [![experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wMath/actions/workflows/wMathPush.yml/badge.svg)](https://github.com/Wandalen/wMath/actions/workflows/wMathPush.yml) [![docs.rs](https://img.shields.io/docsrs/wmath?color=e3e8f0&logo=docs.rs)](https://docs.rs/wmath) [![discord](https://img.shields.io/discord/872391416519737405?color=e3e8f0&logo=discord&logoColor=e3e8f0)](https://discord.gg/JwTG6d2b)

Math library aggregating several math modules.

### Sample

```rust
use wmath::adapter::prelude::*;

let x2_original = wmath::X2::< u8 >( 1, 3 );
println!( "{:?}", x2_original );
/* log : X2(1, 3) */
let x2_to_array = x2_original.clone_as_array();
println!( "{:?}", x2_to_array );
/* log : [1, 3] */
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
