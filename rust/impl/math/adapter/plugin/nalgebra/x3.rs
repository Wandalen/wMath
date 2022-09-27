//!
//! Implement interfaces for objects of the math library.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use core::mem::size_of;
  use crate::prelude::*;
  use crate::{ X3, ScalarInterface };

  impl< Scalar > X3NominalInterface for nalgebra::Vector3< Scalar >
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
  }

  impl< Scalar > X3BasicInterface for nalgebra::Vector3< Scalar >
  where
    Scalar : ScalarInterface,
  {
    #[ inline ]
    fn make( _0 : Self::Scalar, _1 : Self::Scalar, _2 : Self::Scalar ) -> Self
    {
      Self::new( _0, _1, _2 )
    }
  }

  impl< Scalar > X3CanonicalInterface for nalgebra::Vector3< Scalar >
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

    /* */

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
    fn as_canonical( &self ) -> &X3< Self::Scalar >
    {
      debug_assert_eq!( size_of::< Self >(), size_of::< X3< Self::Scalar > >() );
      unsafe
      {
        std::mem::transmute::< _, _ >( self )
      }
    }

    #[ inline ]
    fn as_canonical_mut( &mut self ) -> &mut X3< Self::Scalar >
    {
      debug_assert_eq!( size_of::< Self >(), size_of::< X3< Self::Scalar > >() );
      unsafe
      {
        std::mem::transmute::< _, _ >( self )
      }
    }
  }

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

  //

  impl< Scalar > X3NominalInterface for nalgebra::geometry::Point3< Scalar >
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
  }

  impl< Scalar > X3BasicInterface for nalgebra::geometry::Point3< Scalar >
    where
      Scalar : ScalarInterface,
  {
    #[ inline ]
    fn make( _0 : Self::Scalar, _1 : Self::Scalar, _2 : Self::Scalar ) -> Self
    {
      Self::new( _0, _1, _2 )
    }
  }

  impl< Scalar > X3CanonicalInterface for nalgebra::geometry::Point3< Scalar >
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

    /* */

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
    fn as_canonical( &self ) -> &X3< Self::Scalar >
    {
      debug_assert_eq!( size_of::< Self >(), size_of::< X3< Self::Scalar > >() );
      unsafe
      {
        std::mem::transmute::< _, _ >( self )
      }
    }

    #[ inline ]
    fn as_canonical_mut( &mut self ) -> &mut X3< Self::Scalar >
    {
      debug_assert_eq!( size_of::< Self >(), size_of::< X3< Self::Scalar > >() );
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
