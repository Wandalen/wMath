# module::wmath

Math library aggregating several math modules.

### Sample

<!-- qqq : write --> <!-- aaa : Dmytro : done -->
```rust
use math_adapter::*;

fn main()
{
  let x2_original = x2::< u8 >( 1, 3 );
  println!( "{:?}", x2_original );
  /* log : x2(1, 3) */
  let x2_to_array = x2_original.clone_as_array();
  println!( "{:?}", x2_to_array );
  /* log : [1, 3] */
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
