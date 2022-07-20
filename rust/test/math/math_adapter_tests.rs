// #![ feature( trace_macros ) ]
// #![ feature( type_name_of_val ) ]

#[ allow( unused_imports ) ]
use core::mem::size_of;
#[ allow( unused_imports ) ]
use math_adapter::prelude::*;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

#[ path = "adapter/local_test_tools.rs" ]
mod local_test_tools;
#[ path = "adapter/macro_foreign_x2.rs" ]
mod macro_foreign_x2;
#[ allow( unused_imports ) ]
pub( crate ) use macro_foreign_x2::*;

#[ cfg( feature = "use_std" ) ]
#[ path = "adapter/canonical_x2_test.rs" ]
mod canonical_x2_test;
#[ path = "adapter/helper_test.rs" ]
mod helper_test;
#[ path = "adapter/macro_tools_test.rs" ]
mod macro_tools_test;

#[ cfg( feature = "cgmath" ) ]
#[ path = "adapter/cgmath_test/mod.rs" ]
mod cgmath_test;
#[ cfg( feature = "nalgebra" ) ]
#[ path = "adapter/nalgebra_test/mod.rs" ]
mod nalgebra_test;
#[ cfg( feature = "winit" ) ]
#[ path = "adapter/winit_test/mod.rs" ]
mod winit_test;


