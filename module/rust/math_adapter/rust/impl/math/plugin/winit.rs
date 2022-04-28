/// Internal namespace.
pub mod internal
{
  use core::fmt::Debug;
  use core::hash::Hash;
  use core::ops::Add;
  use core::mem::size_of;
  use crate::prelude::*;

  impl< E > x2_interface for winit::dpi::PhysicalSize< E >
  where
    E :
      Debug +
      PartialEq +
      Copy +
      Clone +
      Hash +
      Sized +
      Add< Output = E > +
    ,
  {
    type Element = E;

    #[ inline ]
    fn make( _0 : Self::Element, _1 : Self::Element ) -> Self
    {
      Self::new( _0, _1 )
    }

    #[ inline ]
    fn _0( &self ) -> Self::Element
    {
      self.width
    }

    #[ inline ]
    fn _1( &self ) -> Self::Element
    {
      self.height
    }

    /* */

    #[ inline ]
    fn _0_ref( &self ) -> &Self::Element
    {
      &self.width
    }

    #[ inline ]
    fn _1_ref( &self ) -> &Self::Element
    {
      &self.height
    }

    /* */

    #[ inline ]
    fn _0_mut( &mut self ) -> &mut Self::Element
    {
      &mut self.width
    }

    #[ inline ]
    fn _1_mut( &mut self ) -> &mut Self::Element
    {
      &mut self.height
    }

    /* */

  }

  impl< E > x2_canonical_interface for x2< E >
  where
    E :
      Debug +
      PartialEq +
      Copy +
      Clone +
      Hash +
      Sized +
      Add< Output = E > +
    ,
  {

    #[ inline ]
    fn as_canonical( &self ) -> &x2< Self::Element >
    {
      debug_assert_eq!( size_of::< Self >(), size_of::< x2< Self::Element > >() );
      unsafe
      {
        std::mem::transmute::< _, _ >( self )
      }
    }

    #[ inline ]
    fn as_canonical_mut( &mut self ) -> &mut x2< Self::Element >
    {
      debug_assert_eq!( size_of::< Self >(), size_of::< x2< Self::Element > >() );
      unsafe
      {
        std::mem::transmute::< _, _ >( self )
      }
    }

  }

}

/// Exposed namespace of the module.
pub mod exposed
{
  // use super::internal as i;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
}
