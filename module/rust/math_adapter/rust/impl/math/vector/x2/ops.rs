
// impl< Right, Scalar > Add< Right > for X2< Scalar >
// where
//   Scalar : ScalarInterface,
//   Right : X2Interface< Scalar = Scalar >,
// {
//   type Output = Self;
//   fn add( self, right : Right ) -> Self::Output
//   {
//     Self::make( self._0() + right._0(), self._1() + right._1() )
//   }
// }
//
// //
//
// impl< Right, Scalar > Add< &Right > for &X2< Scalar >
// where
//   Scalar : ScalarInterface,
//   Right : X2Interface< Scalar = Scalar > + Copy,
// {
//   type Output = < X2< Scalar > as Add< Right > >::Output;
//   fn add( self, right : &Right ) -> Self::Output
//   {
//     Add::< Right >::add( *self, *right )
//   }
// }

// forward_ref_binop! { impl Add, add for $t, $t }
