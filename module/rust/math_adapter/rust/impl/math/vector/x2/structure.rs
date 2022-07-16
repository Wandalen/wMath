/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;


  ///
  /// Vector X2
  ///

  #[ allow( non_camel_case_types ) ]
  #[ repr( C ) ]
  #[ derive( PartialEq, Copy, Clone, Hash, Default ) ]
  pub struct X2
  <
    Scalar : ScalarInterface,
  >
  ( pub Scalar, pub Scalar );

  //

  impl< Scalar > X2NominalInterface for X2< Scalar >
  where
    Scalar : ScalarInterface,
  {
    type Scalar = Scalar;

    #[ inline ]
    fn _0( &self ) -> Self::Scalar
    {
      self.0
    }

    #[ inline ]
    fn _1( &self ) -> Self::Scalar
    {
      self.1
    }

  }

  //

  impl< Scalar > X2BasicInterface for X2< Scalar >
  where
    Scalar : ScalarInterface,
  {

    #[ inline ]
    fn make( _0 : Self::Scalar, _1 : Self::Scalar ) -> Self
    {
      Self( _0, _1 )
    }

  }

  //

  impl< Scalar > X2CanonicalInterface for X2< Scalar >
  where
    Scalar : ScalarInterface,
  {

    /* */

    #[ inline ]
    fn _0_ref( &self ) -> &Self::Scalar
    {
      &self.0
    }

    #[ inline ]
    fn _1_ref( &self ) -> &Self::Scalar
    {
      &self.1
    }

    /* */

    #[ inline ]
    fn _0_mut( &mut self ) -> &mut Self::Scalar
    {
      &mut self.0
    }

    #[ inline ]
    fn _1_mut( &mut self ) -> &mut Self::Scalar
    {
      &mut self.1
    }

    /* */

    #[ inline ]
    fn as_canonical( &self ) -> &X2< Self::Scalar >
    {
      self
    }

    #[ inline ]
    fn as_canonical_mut( &mut self ) -> &mut X2< Self::Scalar >
    {
      self
    }

    /* */

  }

  //

  impl< Scalar > core::fmt::Display for X2< Scalar >
  where
    Scalar : crate::ScalarInterface,
  {
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      write!( f, "X2< {} >( {}, {} )", type_of( &self._0() ), self._0(), self._1() )
    }
  }

  //

  impl< Scalar > core::fmt::Debug for X2< Scalar >
  where
    Scalar : crate::ScalarInterface,
  {
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      write!( f, "X2< {} >( {:?}, {:?} )", type_of( &self._0() ), self._0(), self._1() )
    }
  }

  //

  fn type_of< T >( _ : &T ) -> &'static str
  {
      std::any::type_name::< T >()
  }

  // impl< Scalar, Original > From< &Original >
  // for X2< Scalar >
  // where
  //   Scalar : ScalarInterface,
  //   Original : X2Interface< Scalar = Scalar >,
  // {
  //   fn from( original : &Original ) -> Self
  //   {
  //     Self::make( original._0(), original._1() )
  //   }
  // }

  //

  // impl< Scalar, Original > From< Original >
  // for X2< Scalar >
  // where
  //   Scalar : ScalarInterface,
  //   Original : X2Interface< Scalar = Scalar >,
  // {
  //   fn from( original : Original ) -> Self
  //   {
  //     Self::make( original._0(), original._1() )
  //   }
  // }

}

//

crate::mod_interface!
{
  exposed use X2;
}
