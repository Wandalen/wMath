
///
/// Vector x2
///

#[ allow( non_camel_case_types ) ]
#[ repr( C ) ]
#[ derive( Debug, PartialEq, Copy, Clone, Hash ) ]
pub struct x2
<
  Scalar : ScalarInterface,
>
( pub Scalar, pub Scalar );

//

impl< Scalar > x2_interface for x2< Scalar >
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

}

//

impl< Scalar > x2_canonical_interface for x2< Scalar >
where
  Scalar : ScalarInterface,
{

  #[ inline ]
  fn as_canonical( &self ) -> &x2< Self::Scalar >
  {
    self
  }

  #[ inline ]
  fn as_canonical_mut( &mut self ) -> &mut x2< Self::Scalar >
  {
    self
  }

}
