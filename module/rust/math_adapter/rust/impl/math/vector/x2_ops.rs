
impl< Right, Scalar > Add< Right > for x2< Scalar >
where
  Scalar : ScalarInterface,
  Right : x2_interface< Scalar = Scalar >,
{
  type Output = Self;
  fn add( self, right : Right ) -> Self::Output
  {
    Self::make( self._0() + right._0(), self._1() + right._1() )
  }
}

//

impl< Right, Scalar > Add< &Right > for &x2< Scalar >
where
  Scalar : ScalarInterface,
  Right : x2_interface< Scalar = Scalar > + Copy,
{
  type Output = < x2< Scalar > as Add< Right > >::Output;
  fn add( self, right : &Right ) -> Self::Output
  {
    Add::< Right >::add( *self, *right )
  }
}

// forward_ref_binop! { impl Add, add for $t, $t }
