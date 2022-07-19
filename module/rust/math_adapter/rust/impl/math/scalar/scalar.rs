/// Internal namespace.
pub( crate ) mod private
{
  use core::fmt::{ Debug, Display };
  use core::default::Default;
  // use core::hash::Hash;
  use core::ops::{ Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign };
  use std::cmp::PartialOrd;
  use crate::traits::{ Num, NumCast };

  ///
  /// Traits any element of a vector should implement.
  ///

  pub trait ScalarInterface :
    'static +
    Debug +
    Display +
    PartialEq +
    PartialOrd +
    Default +
    Copy +
    Clone +
    Sized +
    Num +
    NumCast +
    // Hash +
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
    crate::NanLikeInterface +
  {
  }

  impl< T > ScalarInterface for T
  where
    T :
      'static +
      Debug +
      Display +
      PartialEq +
      PartialOrd +
      Default +
      Copy +
      Clone +
      Sized +
      Num +
      NumCast +
      // Hash +
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
      crate::NanLikeInterface +
  {
  }

  //

}

crate::mod_interface!
{
  prelude use ScalarInterface;
}
