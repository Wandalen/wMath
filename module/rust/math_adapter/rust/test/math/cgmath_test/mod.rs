
#[ cfg( feature = "use_std" ) ]
mod x2_test;

#[
  cfg( all
  (
    not( feature = "nalgebra_ops" ),
    not( all( feature = "default_ops", feature = "nalgebra" ) ),
    any( feature = "default_ops", feature = "cgmath_ops" ),
    feature = "use_std",
  ))
]
mod ops_test;
