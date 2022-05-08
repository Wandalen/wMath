// #![ warn( rust_2018_idioms ) ]
// #![ warn( missing_debug_implementations ) ]
// #![ warn( missing_docs ) ]

use cfg_aliases::cfg_aliases;

fn main()
{

  cfg_aliases!
  {
    // Platforms
    wasm: { target_arch = "wasm32" },
    android: { target_os = "android" },
    macos: { target_os = "macos" },
    linux: { target_os = "linux" },
    // Backends
    surfman: { all(unix, feature = "surfman", not(wasm)) },
    glutin: { all(feature = "glutin", not(wasm)) },
    wgl: { all(windows, feature = "wgl", not(wasm)) },
    dummy: { not(any(wasm, glutin, wgl, surfman)) },
  }

}
