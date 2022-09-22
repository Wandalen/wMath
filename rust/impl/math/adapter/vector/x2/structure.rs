//!
//! Implement canonical X2 strcture.
//!

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

  impl_interfaces!
  (
    X2< Scalar >,
    .0,
    .1,
    | _0, _1 | X2::< Scalar > ( _0, _1 ),
    | _self | _self
  );

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
  //   #[ inline ]
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
  //   #[ inline ]
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
