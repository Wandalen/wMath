#![ cfg_attr( not( feature = "use_std" ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/data_type/latest/math_adapter/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Collection of math adapters to decouple your application from math libraries' implementations and to increase inter-libraries compatibility / exchangeability.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

#[ cfg( all
(
  not( feature = "default_ops" ),
  all( feature = "nalgebra_ops", feature = "cgmath_ops" ),
))]
compile_error!( "Only one `*_ops` feature should be enabled. Disable either `nalgebra_ops` or `cgmath_ops`" );

use ::meta_tools::exposed::*;
use core::hash::Hash;
use crate::ScalarInterface;

//

/// Namespace with dependencies.
pub mod dependency
{
  /// General math traits.
  pub use num_traits as traits;
  #[ cfg( feature = "cgmath" )]
  /// Math lib cgmath.
  pub use cgmath as cgmath;
  #[ cfg( feature = "naglebra" )]
  /// Math lib nalgebra.
  pub use naglebra as naglebra;
  #[ cfg( feature = "winit" )]
  /// Math lib winit.
  pub use winit as winit;
}

crate::mod_interface!
{

  exposed use ::num_traits as traits;
  exposed use ::meta_tools::{ for_each, braces_unwrap };

  #[ cfg( all( feature = "cgmath", feature = "use_std" ) ) ]
  exposed use plugin::cgmath;
  #[ cfg( all( feature = "nalgebra", feature = "use_std" ) ) ]
  exposed use plugin::nalgebra;

  /// Meta tools.
  layer meta;
  /// Abstractions.
  layer abs;

  #[ cfg( feature = "use_std" ) ]
  /// Implementation of adapters for specific math libraries.
  layer plugin;
  #[ cfg( feature = "use_std" ) ]
  /// Define scalar interface.
  layer scalar;
  #[ cfg( feature = "use_std" ) ]
  /// Adapters.
  layer vector;

}

// pub use for_each_int;
// pub use for_each_float;
// pub use for_each_number;
