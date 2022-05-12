#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( trace_macros ) ]
// #![ feature( type_name_of_val ) ]

mod test_tools;
mod macro_foreign_x2;
#[ allow( unused_imports ) ]
pub( crate ) use macro_foreign_x2::*;

mod x2_test;
mod helper_test;
mod macro_tools_test;

#[ cfg( feature = "cgmath" )]
mod cgmath_test;
#[ cfg( feature = "nalgebra" )]
mod nalgebra_test;
#[ cfg( feature = "winit" )]
mod winit_test;
