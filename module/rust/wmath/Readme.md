# module::wmath

Math library with adapters.

### Sample

```rust
use wmath::*;

fn main()
{
  let vec2 = ix2_cgmath::new( 1, 3 );
  println!( "{:?}", vec2 );
  /* log : Vector2 [1, 3] */
  let vec3 = vec2.extend( 2 );
  println!( "{:?}", vec3 );
  /* log : Vector3 [1, 3, 2] */
}
```

### To add to your project

```sh
cargo add wmath --dev
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/wmath_trivial
cargo run
```

<!-- qqq : write sample please --> <!-- aaa : Dmytro : done -->

