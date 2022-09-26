#![ cfg_attr( not( feature = "use_std" ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/math_adapter/latest/math_adapter/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Collection of math adapters to decouple your application from math libraries' implementations and to increase inter-libraries compatibility / exchangeability.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#[ cfg( all
(
  not( feature = "default_ops" ),
  all( feature = "nalgebra_ops", feature = "cgmath_ops" ),
))]
compile_error!( "Only one `*_ops` feature should be enabled. Disable either `nalgebra_ops` or `cgmath_ops`" );

use ::meta_tools::exposed::*;
// use core::hash::Hash;
// use crate::ScalarInterface;

//

/// Namespace with dependencies.
pub mod dependency
{
  #[ doc( inline ) ]
  pub use num_traits as traits;
  #[ cfg( feature = "cgmath" )]
  #[ doc( inline ) ]
  pub use cgmath as cgmath;
  #[ cfg( feature = "naglebra" )]
  #[ doc( inline ) ]
  pub use naglebra as naglebra;
  #[ cfg( feature = "winit" )]
  #[ doc( inline ) ]
  pub use winit as winit;
}

crate::mod_interface!
{

  layer meta;
  layer abs;

  #[ cfg( feature = "use_std" ) ]
  layer plugin;
  #[ cfg( feature = "use_std" ) ]
  layer scalar;
  #[ cfg( feature = "use_std" ) ]
  layer vector;
  #[ cfg( feature = "use_std" ) ]
  layer vector_interfaces;

  exposed use ::num_traits as traits;
  exposed use ::meta_tools::{ for_each, braces_unwrap };

  #[ cfg( all( feature = "cgmath", feature = "use_std" ) ) ]
  exposed use super::plugin::cgmath;
  #[ cfg( all( feature = "nalgebra", feature = "use_std" ) ) ]
  exposed use super::plugin::nalgebra;

}
