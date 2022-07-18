
//!
//! Abstractions.
//!

meta_tools::mod_interface!
{

  layer as_foreign;
  layer from;
  layer nan_like;

  #[ cfg( feature = "use_std" ) ]
  layer x2_interface;
}
