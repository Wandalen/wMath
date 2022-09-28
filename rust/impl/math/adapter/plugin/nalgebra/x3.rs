//!
//! Implement interfaces for objects of the math library.
//!

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

  impl_interfaces!
  (
    nalgebra::Vector3< Scalar >,
    .x,
    .y,
    .z,
    | _0, _1, _2 | Self::new( _0, _1, _2 ),
    | _self |
    {
      debug_assert_eq!( size_of::< nalgebra::Vector3< Scalar > >(), size_of::< X3< Scalar > >() );
      unsafe
      {
        std::mem::transmute::< _, _ >( _self )
      }
    }
  );

  impl_interfaces!
  (
    nalgebra::geometry::Point3< Scalar >,
    .x,
    .y,
    .z,
    | _0, _1, _2 | Self::new( _0, _1, _2 ),
    | _self |
    {
      debug_assert_eq!( size_of::< nalgebra::geometry::Point3< Scalar > >(), size_of::< X3< Scalar > >() );
      unsafe
      {
        std::mem::transmute::< _, _ >( _self )
      }
    }
  );

  //

  impl< Scalar, Any > crate::AsNalgebraNonCanonicalInterface< nalgebra::Vector3< Scalar > > for Any
  where
    Scalar : ScalarInterface,
    Any : X3NominalInterface< Scalar = Scalar > + Copy,
  {

    fn clone_as_nalgebra( &self ) -> nalgebra::Vector3< Scalar >
    {
      nalgebra::Vector3::< Scalar >::from2( *self )
    }

  }

  impl< Scalar, Any > crate::AsNalgebraCanonicalInterface< nalgebra::Vector3< Scalar > > for Any
  where
    Scalar : ScalarInterface,
    Any : X3CanonicalInterface< Scalar = Scalar > + Copy,
  {

    fn as_nalgebra( &self ) -> &nalgebra::Vector3< Scalar >
    {
      unsafe
      {
        std::mem::transmute::< _, _ >( self )
      }
    }

    fn as_nalgebra_mut( &mut self ) -> &mut nalgebra::Vector3< Scalar >
    {
      unsafe
      {
        std::mem::transmute::< _, _ >( self )
      }
    }
  }

  impl< Scalar, Original > crate::From2< Original > for nalgebra::Vector3< Scalar >
  where
    Scalar : ScalarInterface,
    Original : X3NominalInterface< Scalar = Scalar >,
  {
    #[ inline ]
    fn from2( original : Original ) -> Self
    {
      nalgebra::Vector3::< Scalar >::new( original._0(), original._1(), original._2() )
    }
  }

  impl< Scalar, Original > crate::From2< Original > for nalgebra::geometry::Point3< Scalar >
  where
    Scalar : ScalarInterface,
    Original : X3NominalInterface< Scalar = Scalar >,
  {
    #[ inline ]
    fn from2( original : Original ) -> Self
    {
      nalgebra::geometry::Point3::< Scalar >::new( original._0(), original._1(), original._2() )
    }
  }
}

crate::mod_interface!
{
}
