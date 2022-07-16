meta_tools::mod_interface!
{

  /// Interfaces to either conver or reinterpret nath data structure as analog of math lib of choice..
  layer as_foreign;
  /// Local implementation of traits From / Into. Required because of limmitations of Rust.
  layer from;
  /// Define several traits like NanLikeInterface.
  layer nan_like;

  /// Interfaces.
  layer x2_interface; // xxx : move to abstract

}
