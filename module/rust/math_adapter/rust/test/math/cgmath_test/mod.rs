
mod x2_test;

/* xxx : use https://crates.io/crates/cfg_aliases */

#[
  cfg( all
  (
    not( feature = "nalgebra_ops" ),
    not( all( feature = "default_ops", feature = "nalgebra" ) ),
    any( feature = "default_ops", feature = "cgmath_ops" ),
  ))
]
mod ops_test;
