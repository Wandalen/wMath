/// Internal namespace.
pub mod internal
{
  use core::fmt::Debug;
  // use core::hash::Hash;
  use core::ops::Add;

  ///
  /// Traits any element of a vector should implement.
  ///

  pub trait ScalarInterface :
    Debug +
    PartialEq +
    Copy +
    Clone +
    // Hash +
    Sized +
    Add< Output = Self > +
  {
  }

  impl< T > ScalarInterface for T
  where
    T :
      Debug +
      PartialEq +
      Copy +
      Clone +
      // Hash +
      Sized +
      Add< Output = Self > +
  {
  }

  //

}

/// Exposed namespace of the module.
pub mod exposed
{
  use super::internal as i;
  pub use i::ScalarInterface;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
  // pub use i::ScalarInterface;
}
