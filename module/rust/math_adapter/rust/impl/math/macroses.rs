/// Internal namespace.
pub mod internal
{

//   ///
//   /// Apply callback to each argument.
//   ///
//
//   #[ macro_export ]
//   macro_rules! apply
//   {
//     ( $Callback:ident, $( $Arg:tt ),* ) =>
//     {
//       $( $Callback!( $Arg ); )*
//     }
//   }
//
//   ///
//   /// For each number call callback.
//   ///
//
//   #[ macro_export ]
//   macro_rules! for_each_number
//   {
//     ( $Callback:ident $( $Prefix:tt )* ) =>
//     {
//       apply!( $Callback, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64 )
//     }
//   }

}

/// Exposed namespace of the module.
pub mod exposed
{
  // use super::internal as i;
  // pub( crate ) use i::apply;
  // pub( crate ) use i::for_each_number;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
}
