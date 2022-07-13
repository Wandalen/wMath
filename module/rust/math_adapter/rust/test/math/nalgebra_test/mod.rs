
#[ cfg( feature = "use_std" ) ]
mod x2_test;
#[
  cfg
  (
    all
    (
      any( feature = "nalgebra_ops", feature = "default_ops" ),
      feature = "use_std",
    )
  )
]
mod ops_test;
