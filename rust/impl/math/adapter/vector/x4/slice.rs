//!
//! Slice.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  impl< Scalar > X4NominalInterface for &[ Scalar ]
    where
      Scalar : ScalarInterface,
  {
    type Scalar = Scalar;

    #[ inline ]
    fn _0( &self ) -> Self::Scalar
    {
      self[ 0 ]
    }

    #[ inline ]
    fn _1( &self ) -> Self::Scalar
    {
      self[ 1 ]
    }

    #[ inline ]
    fn _2( &self ) -> Self::Scalar
    {
      self[ 2 ]
    }

    #[ inline ]
    fn _3( &self ) -> Self::Scalar
    {
      self[ 3 ]
    }

  }

}

//

crate::mod_interface!
{
}
