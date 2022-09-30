//!
//! Implement canonical X3 structure.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  ///
  /// Vector X3
  ///

  #[ allow( non_camel_case_types ) ]
  #[ repr( C ) ]
  #[ derive( PartialEq, Copy, Clone, Hash, Default, VectorInterfaces ) ]
  pub struct X3
  <
    Scalar : ScalarInterface,
  >
  ( pub Scalar, pub Scalar, pub Scalar );

  //

  impl< Scalar > core::fmt::Display for X3< Scalar >
    where
      Scalar : crate::ScalarInterface,
  {
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      write!( f, "X3< {} >( {}, {}, {} )", type_of( &self._0() ), self._0(), self._1(), self._2() )
    }
  }

  //

  impl< Scalar > core::fmt::Debug for X3< Scalar >
    where
      Scalar : crate::ScalarInterface,
  {
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      write!( f, "X3< {} >( {:?}, {:?}, {:?} )", type_of( &self._0() ), self._0(), self._1(), self._2() )
    }
  }

  //

  fn type_of< T >( _ : &T ) -> &'static str
  {
    std::any::type_name::< T >()
  }

  impl< Scalar, Original > crate::From2< Original > for X3< Scalar >
    where
      Scalar : ScalarInterface,
      Original : X3NominalInterface< Scalar = Scalar >,
  {
    #[ inline ]
    fn from2( original : Original ) -> Self
    {
      Self::make( original._0(), original._1(), original._2() )
    }
  }

}

//

crate::mod_interface!
{
  exposed use X3;
}
