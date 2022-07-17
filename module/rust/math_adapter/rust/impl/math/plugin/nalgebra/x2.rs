//!
//! Implement interfaces for objects of the math library.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use core::mem::size_of;
  use crate::prelude::*;
  use crate::{ X2, ScalarInterface };

  macro_rules! impl_x2_for
  {

    ( $Struct1 : ident $( :: $Struct2 : ident )* < $Params : ident > ) =>
    {

      impl< Scalar > X2NominalInterface for $Struct1 $( :: $Struct2 )* < $Params >
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

      }

      impl< Scalar > X2BasicInterface for $Struct1 $( :: $Struct2 )* < $Params >
      where
        Scalar : ScalarInterface,
      {

        #[ inline ]
        fn make( _0 : Self::Scalar, _1 : Self::Scalar ) -> Self
        {
          Self::new( _0, _1 )
        }

      }

      impl< Scalar > X2CanonicalInterface for $Struct1 $( :: $Struct2 )* < $Params >
      where
        Scalar : ScalarInterface,
      {

        /* */

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

        /* */

        #[ inline ]
        fn as_canonical( &self ) -> &X2< Self::Scalar >
        {
          debug_assert_eq!( size_of::< Self >(), size_of::< X2< Self::Scalar > >() );
          unsafe
          {
            std::mem::transmute::< _, _ >( self )
          }
        }

        #[ inline ]
        fn as_canonical_mut( &mut self ) -> &mut X2< Self::Scalar >
        {
          debug_assert_eq!( size_of::< Self >(), size_of::< X2< Self::Scalar > >() );
          unsafe
          {
            std::mem::transmute::< _, _ >( self )
          }
        }

        /* */

      }

      //

    };

  }

  macro_rules! impl_x2_as_for
  {

    ( $Struct1 : ident $( :: $Struct2 : ident )* < $Params : ident > ) =>
    {

      //

      impl< Scalar, Any > crate::AsNalgebraNonCanonicalInterface< $Struct1 $( :: $Struct2 )* < $Params > > for Any
      where
        Scalar : ScalarInterface,
        Any : X2NominalInterface< Scalar = Scalar > + Copy,
      {

        fn clone_as_nalgebra( &self ) -> $Struct1 $( :: $Struct2 )* < $Params >
        {
          $Struct1 $( :: $Struct2 )* :: < $Params > :: from2( *self )
        }

      }

      //

      impl< Scalar, Any > crate::AsNalgebraCanonicalInterface< $Struct1 $( :: $Struct2 )* < $Params > > for Any
      where
        Scalar : ScalarInterface,
        Any : X2CanonicalInterface< Scalar = Scalar > + Copy,
      {

        fn as_nalgebra( &self ) -> &$Struct1 $( :: $Struct2 )* < $Params >
        {
          unsafe
          {
            std::mem::transmute::< _, _ >( self )
          }
        }

        fn as_nalgebra_mut( &mut self ) -> &mut $Struct1 $( :: $Struct2 )* < $Params >
        {
          unsafe
          {
            std::mem::transmute::< _, _ >( self )
          }
        }

      }

    };

  }

  impl_x2_for!( nalgebra::Vector2< Scalar > );
  impl_x2_as_for!( nalgebra::Vector2< Scalar > );
  impl_x2_for!( nalgebra::geometry::Point2< Scalar > );

}

crate::mod_interface!
{
}
