#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( trace_macros ) ]
// #![ feature( type_name_of_val ) ]

use core::mem::size_of;
use math_adapter::prelude::*;
use test_tools::exposed::*;

mod local_test_tools;
mod macro_foreign_x2;
#[ allow( unused_imports ) ]
pub( crate ) use macro_foreign_x2::*;

#[ cfg( feature = "use_std" ) ]
mod canonical_x2_test;
mod helper_test;
mod macro_tools_test;

#[ cfg( feature = "cgmath" )]
mod cgmath_test;
#[ cfg( feature = "nalgebra" )]
mod nalgebra_test;
#[ cfg( feature = "winit" )]
mod winit_test;

