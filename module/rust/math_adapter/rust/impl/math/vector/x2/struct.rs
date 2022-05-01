
///
/// Vector X2
///

#[ allow( non_camel_case_types ) ]
#[ repr( C ) ]
#[ derive( Debug, PartialEq, Copy, Clone, Hash ) ]
pub struct X2
<
  Scalar : ScalarInterface,
>
( pub Scalar, pub Scalar );

//

impl< Scalar > X2Interface for X2< Scalar >
where
  Scalar : ScalarInterface,
{
  type Scalar = Scalar;

  #[ inline ]
  fn make( _0 : Self::Scalar, _1 : Self::Scalar ) -> Self
  {
    Self( _0, _1 )
  }

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

  /* */

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
