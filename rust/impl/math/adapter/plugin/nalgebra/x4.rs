//!
//! Implement interfaces for objects of the math library.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use core::mem::size_of;
  use crate::prelude::*;
  use crate::{ X4, ScalarInterface };

  impl_interfaces!
  (
    nalgebra::Vector4< Scalar >,
    .x,
    .y,
    .z,
    .w,
    | _0, _1, _2, _3 | Self::new( _0, _1, _2, _3 ),
    | _self |
    {
      debug_assert_eq!( size_of::< nalgebra::Vector4< Scalar > >(), size_of::< X4< Scalar > >() );
      unsafe
      {
        std::mem::transmute::< _, _ >( _self )
      }
    }
  );

  impl_interfaces!
  (
    nalgebra::geometry::Point4< Scalar >,
    .x,
    .y,
    .z,
    .w,
    | _0, _1, _2, _3 | Self::new( _0, _1, _2, _3 ),
    | _self |
    {
      debug_assert_eq!( size_of::< nalgebra::geometry::Point4< Scalar > >(), size_of::< X4< Scalar > >() );
      unsafe
      {
        std::mem::transmute::< _, _ >( _self )
      }
    }
  );

  //

  impl< Scalar, Any > crate::AsNalgebraNonCanonicalInterface< nalgebra::Vector4< Scalar > > for Any
  where
    Scalar : ScalarInterface,
    Any : X4NominalInterface< Scalar = Scalar > + Copy,
  {

    fn clone_as_nalgebra( &self ) -> nalgebra::Vector4< Scalar >
    {
      nalgebra::Vector4::< Scalar >::from2( *self )
    }

  }

  impl< Scalar, Any > crate::AsNalgebraCanonicalInterface< nalgebra::Vector4< Scalar > > for Any
  where
    Scalar : ScalarInterface,
    Any : X4CanonicalInterface< Scalar = Scalar > + Copy,
  {

    fn as_nalgebra( &self ) -> &nalgebra::Vector4< Scalar >
    {
      unsafe
      {
        std::mem::transmute::< _, _ >( self )
      }
    }

    fn as_nalgebra_mut( &mut self ) -> &mut nalgebra::Vector4< Scalar >
    {
      unsafe
      {
        std::mem::transmute::< _, _ >( self )
      }
    }
  }

  //

  impl< Scalar, Original > crate::From2< Original > for nalgebra::Vector4< Scalar >
  where
    Scalar : ScalarInterface,
    Original : X4NominalInterface< Scalar = Scalar >,
  {
    #[ inline ]
    fn from2( original : Original ) -> Self
    {
      nalgebra::Vector4::< Scalar >::new( original._0(), original._1(), original._2(), original._3() )
    }
  }

  impl< Scalar, Original > crate::From2< Original > for nalgebra::geometry::Point4< Scalar >
  where
    Scalar : ScalarInterface,
    Original : X4NominalInterface< Scalar = Scalar >,
  {
    #[ inline ]
    fn from2( original : Original ) -> Self
    {
      nalgebra::geometry::Point4::< Scalar >::new( original._0(), original._1(), original._2(), original._3() )
    }
  }
}

crate::mod_interface!
{
}
