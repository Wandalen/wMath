/// Internal namespace.
pub( crate ) mod private
{
  use core::mem::size_of;
  use crate::prelude::*;
  use crate::
  {
    X4,
    ScalarInterface
  };

  //

  impl< Scalar > X4NominalInterface for cgmath::Vector4< Scalar >
    where
      Scalar : ScalarInterface,
  {
    type Scalar = Scalar;

    #[ inline ]
    fn _0( &self ) -> Self::Scalar
    {
      self.x
    }

    #[ inline ]
    fn _1( &self ) -> Self::Scalar
    {
      self.y
    }

    #[ inline ]
    fn _2( &self ) -> Self::Scalar
    {
      self.z
    }

    #[ inline ]
    fn _3( &self ) -> Self::Scalar
    {
      self.w
    }
  }

  impl< Scalar > X4BasicInterface for cgmath::Vector4< Scalar >
    where
      Scalar : ScalarInterface,
  {
    #[ inline ]
    fn make( _0 : Self::Scalar, _1 : Self::Scalar, _2 : Self::Scalar, _3 : Self::Scalar ) -> Self
    {
      Self::new( _0, _1, _2, _3 )
    }
  }

  impl< Scalar > X4CanonicalInterface for cgmath::Vector4< Scalar >
    where
      Scalar : ScalarInterface,
  {
    #[ inline ]
    fn _0_ref( &self ) -> &Self::Scalar
    {
      &self.x
    }

    #[ inline ]
    fn _1_ref( &self ) -> &Self::Scalar
    {
      &self.y
    }

    #[ inline ]
    fn _2_ref( &self ) -> &Self::Scalar
    {
      &self.z
    }

    #[ inline ]
    fn _3_ref( &self ) -> &Self::Scalar
    {
      &self.w
    }

    #[ inline ]
    fn _0_mut( &mut self ) -> &mut Self::Scalar
    {
      &mut self.x
    }

    #[ inline ]
    fn _1_mut( &mut self ) -> &mut Self::Scalar
    {
      &mut self.y
    }

    #[ inline ]
    fn _2_mut( &mut self ) -> &mut Self::Scalar
    {
      &mut self.z
    }

    #[ inline ]
    fn _3_mut( &mut self ) -> &mut Self::Scalar
    {
      &mut self.w
    }

    #[ inline ]
    fn as_canonical( &self ) -> &X4< Self::Scalar >
    {
      debug_assert_eq!( size_of::< Self >(), size_of::< X4< Self::Scalar > >() );
      unsafe
        {
          std::mem::transmute::< _, _ >( self )
        }
    }

    #[ inline ]
    fn as_canonical_mut( &mut self ) -> &mut X4< Self::Scalar >
    {
      debug_assert_eq!( size_of::< Self >(), size_of::< X4< Self::Scalar > >() );
      unsafe
        {
          std::mem::transmute::< _, _ >( self )
        }
    }
  }

  //

  impl< Scalar, Any > crate::AsCgmathNonCanonicalInterface< cgmath::Vector4< Scalar > >
  for Any
    where
      Scalar : ScalarInterface,
      Any : X4NominalInterface< Scalar = Scalar > + Copy,
  {

    fn clone_as_cgmath( &self ) -> cgmath::Vector4< Scalar >
    {
      cgmath::Vector4::< Scalar >::from2( *self )
    }

  }

  impl< Scalar, Any > crate::AsCgmathCanonicalInterface< cgmath::Vector4< Scalar > >
  for Any
    where
      Scalar : ScalarInterface,
      Any : X4CanonicalInterface< Scalar = Scalar > + Copy,
  {
    fn as_cgmath( &self ) -> &cgmath::Vector4< Scalar >
    {
      unsafe
        {
          std::mem::transmute::< _, _ >( self )
        }
    }

    fn as_cgmath_mut( &mut self ) -> &mut cgmath::Vector4< Scalar >
    {
      unsafe
        {
          std::mem::transmute::< _, _ >( self )
        }
    }

  }

  impl< Scalar, Original > crate::From2< Original > for cgmath::Vector4< Scalar >
    where
      Scalar : ScalarInterface,
      Original : X4NominalInterface< Scalar = Scalar >,
  {
    #[ inline ]
    fn from2( original : Original ) -> Self
    {
      cgmath::Vector4::< Scalar >::new( original._0(), original._1(), original._2(), original._3() )
    }
  }
}

crate::mod_interface!
{

}