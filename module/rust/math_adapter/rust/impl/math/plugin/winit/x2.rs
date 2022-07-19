
//!
//! Implement X2 interfaces for vectors of the math lib.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use core::mem::size_of;
  use crate::prelude::*;
  use crate::{ X2, ScalarInterface };

  ///
  /// Implements interfaces for vector X2;
  ///

  macro_rules! impl_x2_for
  {
    ( $Struct : path, $_0 : ident, $_1 : ident ) =>
    {

      impl< Scalar > X2NominalInterface for $Struct
      where
        Scalar : ScalarInterface,
      {
        type Scalar = Scalar;

        #[ inline ]
        fn _0( &self ) -> Self::Scalar
        {
          self.$_0
        }

        #[ inline ]
        fn _1( &self ) -> Self::Scalar
        {
          self.$_1
        }

      }

      impl< Scalar > X2BasicInterface for $Struct
      where
        Scalar : ScalarInterface,
      {

        #[ inline ]
        fn make( _0 : Self::Scalar, _1 : Self::Scalar ) -> Self
        {
          Self::new( _0, _1 )
        }

      }

      impl< Scalar > X2CanonicalInterface for $Struct
      where
        Scalar : ScalarInterface,
      {

        /* */

        #[ inline ]
        fn _0_ref( &self ) -> &Self::Scalar
        {
          &self.$_0
        }

        #[ inline ]
        fn _1_ref( &self ) -> &Self::Scalar
        {
          &self.$_1
        }

        /* */

        #[ inline ]
        fn _0_mut( &mut self ) -> &mut Self::Scalar
        {
          &mut self.$_0
        }

        #[ inline ]
        fn _1_mut( &mut self ) -> &mut Self::Scalar
        {
          &mut self.$_1
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

      }

    };

  }

  impl_x2_for!( winit::dpi::PhysicalSize< Scalar >, width, height );
  impl_x2_for!( winit::dpi::LogicalSize< Scalar >, width, height );
  impl_x2_for!( winit::dpi::PhysicalPosition< Scalar >, x, y );
  impl_x2_for!( winit::dpi::LogicalPosition< Scalar >, x, y );

}

//

crate::mod_interface!
{
}
