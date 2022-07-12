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

/// General math traits.
pub use num_traits as traits;

pub use ::meta_tools::prelude::*;
// pub use meta_tools::mod_interface;

/// Macro tools, to be moved out.
mod macro_tools;
pub use macro_tools::*;
/// Macro helpers, for example to enumarate number types.
mod macro_helper;
pub use macro_helper::*;

/// Interfaces to either conver or reinterpret nath data structure as analog of math lib of choice..
pub mod as_foreign;
/// Local implementation of traits From / Into. Required because of limmitations of Rust.
pub mod from;

/// Implementation of adapters for specific math libraries.
pub mod plugin;
pub use plugin::*;
/// Define scalar interface.
pub mod scalar;
/// Define several traits like NanLikeInterface.
pub mod interfaces;
/// Adapters.
pub mod vector;

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

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::as_foreign::exposed::*;
  #[ doc( inline ) ]
  pub use super::from::exposed::*;
  #[ doc( inline ) ]
  pub use super::plugin::exposed::*;
  #[ doc( inline ) ]
  pub use super::scalar::exposed::*;
  #[ doc( inline ) ]
  pub use super::interfaces::exposed::*;
  #[ doc( inline ) ]
  pub use super::vector::exposed::*;
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::as_foreign::prelude::*;
  pub use super::from::prelude::*;
  pub use super::plugin::prelude::*;
  pub use super::scalar::prelude::*;
  pub use super::interfaces::prelude::*;
  pub use super::vector::prelude::*;
}
