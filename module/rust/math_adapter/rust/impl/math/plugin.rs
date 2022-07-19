//!
//! Implementation of adapters for specific math libraries.
//!

meta_tools::mod_interface!
{
  #[ cfg( feature = "cgmath" ) ]
  layer cgmath;
  #[ cfg( feature = "nalgebra" ) ]
  layer nalgebra;
  #[ cfg( feature = "winit" ) ]
  layer winit;
}
