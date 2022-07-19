
use cfg_aliases::cfg_aliases;

fn main()
{

  cfg_aliases!
  {
    // platform
    wasm : { target_arch = "wasm32" },
    android : { target_os = "android" },
    macos : { target_os = "macos" },
    linux : { target_os = "linux" },
    // etc
    nalgebra_ops :
    {
      any( feature = "nalgebra_ops", feature = "default_ops" )
    },
    cgmath_ops :
    {
      all
      (
        not( feature = "nalgebra_ops" ),
        not( all( feature = "default_ops", feature = "nalgebra" ) ),
        any( feature = "default_ops", feature = "cgmath_ops" )
      )
    },
  }

}

// #[ cfg( any( feature = "nalgebra_ops", feature = "default_ops" ) ) ]

// #[
//   cfg( all
//   (
//     not( feature = "nalgebra_ops" ),
//     not( all( feature = "default_ops", feature = "nalgebra" ) ),
//     any( feature = "default_ops", feature = "cgmath_ops" ),
//   ))
// ]
