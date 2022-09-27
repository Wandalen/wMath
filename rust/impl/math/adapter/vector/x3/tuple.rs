//!
//! Implement interfaces for tuple.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;


  impl< Scalar > X3NominalInterface for ( Scalar, Scalar, Scalar )
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

    #[ inline ]
    fn _2( &self ) -> Self::Scalar
    {
      self.2
    }

  }

  //

  impl< Scalar > X3BasicInterface for ( Scalar, Scalar, Scalar )
    where
      Scalar : ScalarInterface,
  {

    #[ inline ]
    fn make( _0 : Self::Scalar, _1 : Self::Scalar, _2 : Self::Scalar ) -> Self
    {
      ( _0, _1, _2 )
    }

  }

  //

  impl< Scalar > X3CanonicalInterface for ( Scalar, Scalar, Scalar )
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

    #[ inline ]
    fn _2_ref( &self ) -> &Self::Scalar
    {
      &self.2
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

    #[ inline ]
    fn _2_mut( &mut self ) -> &mut Self::Scalar
    {
      &mut self.2
    }

    /* */

    #[ inline ]
    fn as_canonical( &self ) -> &X3< Self::Scalar >
    {
      unsafe
        {
          std::mem::transmute::< _, _ >( self )
        }
    }

    #[ inline ]
    fn as_canonical_mut( &mut self ) -> &mut X3< Self::Scalar >
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
