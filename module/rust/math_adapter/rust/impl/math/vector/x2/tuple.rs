//!
//! Implement interfaces for tuple.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;


  impl< Scalar > X2NominalInterface for ( Scalar, Scalar )
  where
    Scalar : ScalarInterface,
  {
    type Scalar = Scalar;

    #[ inline ]
    fn _0( &self ) -> Self::Scalar
    {
      self.0
    }

    #[ inline ]
    fn _1( &self ) -> Self::Scalar
    {
      self.1
    }

  }

  //

  impl< Scalar > X2BasicInterface for ( Scalar, Scalar )
  where
    Scalar : ScalarInterface,
  {

    #[ inline ]
    fn make( _0 : Self::Scalar, _1 : Self::Scalar ) -> Self
    {
      ( _0, _1 )
    }

  }

  //

  impl< Scalar > X2CanonicalInterface for ( Scalar, Scalar )
  where
    Scalar : ScalarInterface,
  {

    /* */

    #[ inline ]
    fn _0_ref( &self ) -> &Self::Scalar
    {
      &self.0
    }

    #[ inline ]
    fn _1_ref( &self ) -> &Self::Scalar
    {
      &self.1
    }

    /* */

    #[ inline ]
    fn _0_mut( &mut self ) -> &mut Self::Scalar
    {
      &mut self.0
    }

    #[ inline ]
    fn _1_mut( &mut self ) -> &mut Self::Scalar
    {
      &mut self.1
    }

    /* */

    #[ inline ]
    fn as_canonical( &self ) -> &X2< Self::Scalar >
    {
      unsafe
      {
        std::mem::transmute::< _, _ >( self )
      }
    }

    #[ inline ]
    fn as_canonical_mut( &mut self ) -> &mut X2< Self::Scalar >
    {
      unsafe
      {
        std::mem::transmute::< _, _ >( self )
      }
    }

    /* */

  }

}

//

crate::mod_interface!
{
}
