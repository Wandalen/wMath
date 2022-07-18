/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  impl< Scalar > X2NominalInterface for &[ Scalar ]
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

  }

}

//

crate::mod_interface!
{
}
