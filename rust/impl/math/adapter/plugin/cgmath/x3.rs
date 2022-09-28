/// Internal namespace.
pub( crate ) mod private
{
  use core::mem::size_of;
  use crate::prelude::*;
  use crate::
  {
    X3,
    ScalarInterface
  };

  //

  impl_interfaces!
  (
    cgmath::Vector3< Scalar >,
    .x,
    .y,
    .z,
    | _0, _1, _2 | Self::new( _0, _1, _2 ),
    | _self |
    {
      debug_assert_eq!( size_of::< cgmath::Vector3< Scalar > >(), size_of::< X3< Scalar > >() );
      unsafe
      {
        std::mem::transmute::< _, _ >( _self )
      }
    }
  );

  impl_interfaces!
  (
    cgmath::Point3< Scalar >,
    .x,
    .y,
    .z,
    | _0, _1, _2 | Self::new( _0, _1, _2 ),
    | _self |
    {
      debug_assert_eq!( size_of::< cgmath::Point3< Scalar > >(), size_of::< X3< Scalar > >() );
      unsafe
      {
        std::mem::transmute::< _, _ >( _self )
      }
    }
  );

  //

  impl< Scalar, Any > crate::AsCgmathNonCanonicalInterface< cgmath::Vector3< Scalar > >
  for Any
  where
    Scalar : ScalarInterface,
    Any : X3NominalInterface< Scalar = Scalar > + Copy,
  {

  fn clone_as_cgmath( &self ) -> cgmath::Vector3< Scalar >
  {
    cgmath::Vector3::< Scalar >::from2( *self )
  }

  }

  impl< Scalar, Any > crate::AsCgmathCanonicalInterface< cgmath::Vector3< Scalar > >
  for Any
  where
    Scalar : ScalarInterface,
    Any : X3CanonicalInterface< Scalar = Scalar > + Copy,
  {
  fn as_cgmath( &self ) -> &cgmath::Vector3< Scalar >
  {
    unsafe
    {
      std::mem::transmute::< _, _ >( self )
    }
  }

  fn as_cgmath_mut( &mut self ) -> &mut cgmath::Vector3< Scalar >
  {
    unsafe
    {
      std::mem::transmute::< _, _ >( self )
    }
  }

  }

  impl< Scalar, Original > crate::From2< Original > for cgmath::Vector3< Scalar >
  where
    Scalar : ScalarInterface,
    Original : X3NominalInterface< Scalar = Scalar >,
  {
    #[ inline ]
    fn from2( original : Original ) -> Self
    {
      cgmath::Vector3::< Scalar >::new( original._0(), original._1(), original._2() )
    }
  }

  impl< Scalar, Original > crate::From2< Original > for cgmath::Point3< Scalar >
  where
    Scalar : ScalarInterface,
    Original : X3NominalInterface< Scalar = Scalar >,
  {
    #[ inline ]
    fn from2( original : Original ) -> Self
    {
      cgmath::Point3::< Scalar >::new( original._0(), original._1(), original._2() )
    }
  }
}

crate::mod_interface!
{

}