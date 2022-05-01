/// Internal namespace.
pub mod internal
{
  use core::fmt::Debug;
  // use core::hash::Hash;
  use core::ops::{ Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign };
  use std::cmp::PartialOrd;
  use crate::traits::{ Num, NumCast };

  ///
  /// Traits any element of a vector should implement.
  ///

  pub trait ScalarInterface :
    Debug +
    PartialEq +
    PartialOrd +
    Copy +
    Clone +
    // Hash +
    Sized +
    Num +
    NumCast +
    // Not< Output = Self > +
    // Neg< Output = Self > +
    Add< Output = Self > +
    Sub< Output = Self > +
    Mul< Output = Self > +
    Div< Output = Self > +
    Rem< Output = Self > +
    AddAssign< Self > +
    SubAssign< Self > +
    MulAssign< Self > +
    DivAssign< Self > +
    RemAssign< Self > +
  {
  }

  impl< T > ScalarInterface for T
  where
    T :
      Debug +
      PartialEq +
      PartialOrd +
      Copy +
      Clone +
      // Hash +
      Sized +
      Num +
      NumCast +
      // Not< Output = Self > +
      // Neg< Output = Self > +
      Add< Output = Self > +
      Sub< Output = Self > +
      Mul< Output = Self > +
      Div< Output = Self > +
      Rem< Output = Self > +
      AddAssign< Self > +
      SubAssign< Self > +
      MulAssign< Self > +
      DivAssign< Self > +
      RemAssign< Self > +
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
  use super::internal as i;
  pub use i::ScalarInterface;
}
