#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

mod tools;
mod x2_with_records;

mod x2_test;
mod experiment_test;

#[cfg( feature = "winit" )]
mod winit_test;
#[cfg( feature = "cgmath" )]
mod cgmath_test;
