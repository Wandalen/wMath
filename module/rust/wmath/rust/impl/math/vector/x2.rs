/// Internal namespace.
pub mod internal
{
  use core::fmt::Debug;
  use core::hash::Hash;
  use core::ops::Add;

  ///
  /// Traits any element of a vector should implement.
  ///

  pub trait ScalarInterface :
    Debug +
    PartialEq +
    Copy +
    Clone +
    // Hash +
    Sized +
    Add< Output = Self > +
  {
  }

  impl< T > ScalarInterface for T
  where
    T :
      Debug +
      PartialEq +
      Copy +
      Clone +
      // Hash +
      Sized +
      Add< Output = Self > +
  {
  }

  ///
  /// Vector x2
  ///

  #[ allow( non_camel_case_types ) ]
  #[ repr( C ) ]
  #[ derive( Debug, PartialEq, Copy, Clone, Hash ) ]
  pub struct x2
  <
    E : ScalarInterface,
  >
  ( pub E, pub E );

  ///
  /// Interface of vector x2.
  ///

  #[ allow( non_camel_case_types ) ]
  pub trait x2_interface
  {
    /// Type of element.
    type Element : ScalarInterface;

    /// Constructor.
    fn make( _0 : Self::Element, _1 : Self::Element ) -> Self;

    /// First element.
    fn _0( &self ) -> Self::Element;
    /// Second element.
    fn _1( &self ) -> Self::Element;
    /// First element.
    #[ inline ]
    fn x( &self ) -> Self::Element
    {
      self._0()
    }
    /// Second element.
    #[ inline ]
    fn y( &self ) -> Self::Element
    {
      self._1()
    }

    /// First element.
    fn _0_ref( &self ) -> &Self::Element;
    /// Second element.
    fn _1_ref( &self ) -> &Self::Element;
    /// First element.
    #[ inline ]
    fn x_ref( &self ) -> &Self::Element
    {
      self._0_ref()
    }
    /// Second element.
    #[ inline ]
    fn y_ref( &self ) -> &Self::Element
    {
      self._1_ref()
    }

    /// First element.
    fn _0_mut( &mut self ) -> &mut Self::Element;
    /// Second element.
    fn _1_mut( &mut self ) -> &mut Self::Element;
    /// First element.
    #[ inline ]
    fn x_mut( &mut self ) -> &mut Self::Element
    {
      self._0_mut()
    }
    /// Second element.
    #[ inline ]
    fn y_mut( &mut self ) -> &mut Self::Element
    {
      self._1_mut()
    }

    /// Clone as tuple.
    #[ inline ]
    fn clone_as_tuple( &self ) -> ( Self::Element , Self::Element )
    {
      ( self._0(), self._1() )
    }
    /// Clone as array.
    #[ inline ]
    fn clone_as_array( &self ) -> [ Self::Element ; 2 ]
    {
      [ self._0(), self._1() ]
    }
    /// Clone as canonical.
    #[ inline ]
    fn clone_as_canonical( &self ) -> x2< Self::Element >
    {
      x2::< Self::Element >( self._0(), self._1() )
    }

  }

  ///
  /// Interface of vector x2 for structures with the canonical layout.
  ///

  #[ allow( non_camel_case_types ) ]
  pub trait x2_canonical_interface : x2_interface + Sized
  {

    /// Interpret as tuple.
    #[ inline ]
    fn as_tuple( &self ) -> &( Self::Element , Self::Element )
    {
      debug_assert_eq!( core::mem::size_of::< Self >(), core::mem::size_of::< ( Self::Element , Self::Element ) >() );
      unsafe
      {
        std::mem::transmute::< _, _ >( self.as_canonical() )
      }
    }
    /// Interpret as array.
    #[ inline ]
    fn as_array( &self ) -> &[ Self::Element ; 2 ]
    {
      unsafe
      {
        std::mem::transmute::< _, _ >( self.as_canonical() )
      }
    }
    /// Interpret as slice.
    #[ inline ]
    fn as_slice( &self ) -> &[ Self::Element ]
    {
      &self.as_array()[ .. ]
    }
    /// Canonical representation of the vector.
    fn as_canonical( &self ) -> &x2< Self::Element >;

    /// Interpret as mutable tuple.
    #[ inline ]
    fn as_tuple_mut( &mut self ) -> &mut ( Self::Element , Self::Element )
    {
      debug_assert_eq!( core::mem::size_of::< Self >(), core::mem::size_of::< ( Self::Element , Self::Element ) >() );
      unsafe
      {
        std::mem::transmute::< _, _ >( self.as_canonical_mut() )
      }
    }
    /// Interpret as mutable array.
    #[ inline ]
    fn as_array_mut( &mut self ) -> &mut [ Self::Element ; 2 ]
    {
      unsafe
      {
        std::mem::transmute::< _, _ >( self.as_canonical_mut() )
      }
    }
    /// Interpret as mutable slice.
    #[ inline ]
    fn as_slice_mut( &mut self ) -> &mut [ Self::Element ]
    {
      &mut self.as_array_mut()[ .. ]
    }
    /// Mutable canonical representation of the vector.
    fn as_canonical_mut( &mut self ) -> &mut x2< Self::Element >;

  }

  impl< E > x2_interface for x2< E >
  where
    E : ScalarInterface,
  {
    type Element = E;

    #[ inline ]
    fn make( _0 : Self::Element, _1 : Self::Element ) -> Self
    {
      Self( _0, _1 )
    }

    #[ inline ]
    fn _0( &self ) -> Self::Element
    {
      self.0
    }

    #[ inline ]
    fn _1( &self ) -> Self::Element
    {
      self.1
    }

    /* */

    #[ inline ]
    fn _0_ref( &self ) -> &Self::Element
    {
      &self.0
    }

    #[ inline ]
    fn _1_ref( &self ) -> &Self::Element
    {
      &self.1
    }

    /* */

    #[ inline ]
    fn _0_mut( &mut self ) -> &mut Self::Element
    {
      &mut self.0
    }

    #[ inline ]
    fn _1_mut( &mut self ) -> &mut Self::Element
    {
      &mut self.1
    }

    /* */

  }

  //

  impl< E > x2_canonical_interface for x2< E >
  where
    E : ScalarInterface,
  {

    #[ inline ]
    fn as_canonical( &self ) -> &x2< Self::Element >
    {
      self
    }

    #[ inline ]
    fn as_canonical_mut( &mut self ) -> &mut x2< Self::Element >
    {
      self
    }

  }

  //

  impl< Right, E > Add< Right > for x2< E >
  where
    E : ScalarInterface,
    Right : x2_interface< Element = E >,
  {
    type Output = Self;
    fn add( self, right : Right ) -> Self::Output
    {
      Self::make( self._0() + right._0(), self._1() + right._1() )
    }
  }

  //

  impl< Right, E > Add< &Right > for &x2< E >
  where
    E : ScalarInterface,
    Right : x2_interface< Element = E > + Copy,
  {
    type Output = < x2< E > as Add< Right > >::Output;
    fn add( self, right : &Right ) -> Self::Output
    {
      Add::< Right >::add( *self, *right )
    }
  }

  // forward_ref_binop! { impl Add, add for $t, $t }

}

/// Exposed namespace of the module.
pub mod exposed
{
  use super::internal as i;
  pub use i::ScalarInterface;
  pub use i::x2;
  pub use i::x2_interface;
  pub use i::x2_canonical_interface;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  use super::internal as i;
  pub use i::x2_interface;
  pub use i::x2_canonical_interface;
}
