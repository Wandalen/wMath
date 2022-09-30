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
#[ path = "adapter/macro_foreign_x3.rs" ]
mod macro_foreign_x3;
#[ path = "adapter/macro_foreign_x4.rs" ]
mod macro_foreign_x4;
#[ allow( unused_imports ) ]
pub( crate ) use macro_foreign_x2::*;
#[ allow( unused_imports ) ]
pub( crate ) use macro_foreign_x3::*;
#[ allow( unused_imports ) ]
pub( crate ) use macro_foreign_x4::*;

#[ cfg( feature = "use_std" ) ]
#[ path = "adapter/canonical_x2_test.rs" ]
mod canonical_x2_test;
#[ cfg( feature = "use_std" ) ]
#[ path = "adapter/canonical_x3_test.rs" ]
mod canonical_x3_test;
#[ cfg( feature = "use_std" ) ]
#[ path = "adapter/canonical_x4_test.rs" ]
mod canonical_x4_test;
#[ path = "adapter/helper_test.rs" ]
mod helper_test;
#[ path = "adapter/macro_tools_test.rs" ]
mod macro_tools_test;
#[ cfg( feature = "use_std" ) ]
#[ path = "adapter/impl_interfaces_macro_test.rs" ]
mod impl_interfaces_macro_test;
#[ cfg( feature = "use_std" ) ]
#[ path = "adapter/derive_vector_interfaces_test.rs"]
mod derive_vector_interfaces_test;

#[ cfg( feature = "cgmath" ) ]
#[ path = "adapter/cgmath_test/mod.rs" ]
mod cgmath_test;
#[ cfg( feature = "nalgebra" ) ]
#[ path = "adapter/nalgebra_test/mod.rs" ]
mod nalgebra_test;
#[ cfg( feature = "winit" ) ]
#[ path = "adapter/winit_test/mod.rs" ]
mod winit_test;

