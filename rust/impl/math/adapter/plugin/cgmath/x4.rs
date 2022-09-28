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

  impl_interfaces!
  (
    cgmath::Vector4< Scalar >,
    .x,
    .y,
    .z,
    .w,
    | _0, _1, _2, _3 | Self::new( _0, _1, _2, _3 ),
    | _self |
    {
      debug_assert_eq!( size_of::< cgmath::Vector4< Scalar > >(), size_of::< X4< Scalar > >() );
      unsafe
      {
        std::mem::transmute::< _, _ >( _self )
      }
    }
  );

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