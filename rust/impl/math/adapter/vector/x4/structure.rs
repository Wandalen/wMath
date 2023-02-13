//!
//! Implement canonical X4 structure.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  ///
  /// Vector X4
  ///

  #[ allow( non_camel_case_types ) ]
  #[ repr( C ) ]
  #[ derive( PartialEq, Copy, Clone, Hash, Default, VectorInterfaces ) ]
  pub struct X4
  <
    Scalar : ScalarInterface,
  >
  ( pub Scalar, pub Scalar, pub Scalar, pub Scalar );

  //

  impl< Scalar > core::fmt::Display for X4< Scalar >
    where
      Scalar : crate::ScalarInterface,
  {
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      write!( f, "X4< {} >( {}, {}, {}, {} )", type_of( &self._0() ), self._0(), self._1(), self._2(), self._3() )
    }
  }

  //

  impl< Scalar > core::fmt::Debug for X4< Scalar >
    where
      Scalar : crate::ScalarInterface,
  {
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      write!( f, "X4< {} >( {:?}, {:?}, {:?}, {:?} )", type_of( &self._0() ), self._0(), self._1(), self._2(), self._3() )
    }
  }

  //

  fn type_of< T >( _ : &T ) -> &'static str
  {
    std::any::type_name::< T >()
  }

  impl< Scalar, Original > crate::From2< Original > for X4< Scalar >
    where
      Scalar : ScalarInterface,
      Original : X4NominalInterface< Scalar = Scalar >,
  {
    #[ inline ]
    fn from2( original : Original ) -> Self
    {
      Self::make( original._0(), original._1(), original._2(), original._3() )
    }
  }
}

//

crate::mod_interface!
{
  exposed use X4;
}
