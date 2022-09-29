//!
//! Implement canonical X2 strcture.
//!
#![ allow( clippy::just_underscores_and_digits ) ]

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  ///
  /// Vector X2
  ///

  #[ allow( non_camel_case_types ) ]
  #[ repr( C ) ]
  #[ derive( PartialEq, Eq, Copy, Clone, Hash, Default, VectorInterfaces ) ]
  pub struct X2
  <
    Scalar : ScalarInterface,
  >
  ( pub Scalar, pub Scalar );

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
