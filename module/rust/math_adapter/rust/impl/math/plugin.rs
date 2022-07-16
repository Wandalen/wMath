meta_tools::mod_interface!
{
  /// Implement adapters for `cgmath`.
  #[ cfg( feature = "cgmath" ) ]
  layer cgmath;
  /// Implement adapters for `nalgebra`.
  #[ cfg( feature = "nalgebra" ) ]
  layer nalgebra;
  /// Implement adapters for `winit`.
  #[ cfg( feature = "winit" ) ]
  layer winit;
}
