#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( trace_macros ) ]
// #![ feature( type_name_of_val ) ]

mod test_tools;
mod macro_x2;
pub( crate ) use macro_x2::*;

mod x2_test;
mod experiment_test;

#[ cfg( feature = "cgmath" )]
mod cgmath_test;
#[ cfg( feature = "nalgebra" )]
mod nalgebra_test;
#[ cfg( feature = "winit" )]
mod winit_test;
