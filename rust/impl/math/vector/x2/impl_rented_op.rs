/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  ///
  /// Declare operation with 1 operand renting its implementation from math lib of choice.
  ///

  #[ allow( unused_macros ) ]
  #[ macro_export ]
  macro_rules! impl_x2_rented_op1
  {

    () => {};

    ( $Op : ident, $op : ident, $Va : ident $( :: $Vb : ident )* ) =>
    {

      impl< Scalar > $Op for X2< Scalar >
      where
        Scalar : ScalarInterface + $Op< Output = Scalar >,
      {
        type Output = Self;
        #[ inline ]
        fn $op( self  ) -> Self::Output
        {
          $Op::$op
          (
            $Va $( :: $Vb )*::< Scalar >::from2( self ),
          ).into2()
        }
      }

      //

      impl< Scalar > $Op for &X2< Scalar >
      where
        Scalar : ScalarInterface + $Op< Output = Scalar >,
      {
        type Output = < X2< Scalar > as $Op >::Output;
        #[ inline ]
        fn $op( self ) -> Self::Output
        {
          $Op::$op( *self )
        }
      }

    };

  }

  ///
  /// Declare operation with 2 operands renting its implementation from math lib of choice.
  ///

  #[ allow( unused_macros ) ]
  #[ macro_export ]
  macro_rules! impl_x2_rented_op2
  {

    () => {};

    ( $Op : ident, $op : ident, $Va : ident $( :: $Vb : ident )* ) =>
    {

      impl< Right, Scalar > $Op< Right > for X2< Scalar >
      where
        Scalar : ScalarInterface + $Op< Output = Scalar >,
        Right : X2Interface< Scalar = Scalar > + Copy,
      {
        type Output = Self;
        #[ inline ]
        fn $op( self, right : Right ) -> Self::Output
        {
          $Op::< $Va $( :: $Vb )*< Scalar > >::$op
          (
            $Va $( :: $Vb )*::< Scalar >::from2( self ),
            $Va $( :: $Vb )*::< Scalar >::from2( right ),
          ).into2()
        }
      }

      //

      impl< Right, Scalar > $Op< &Right > for &X2< Scalar >
      where
        Scalar : ScalarInterface + $Op< Output = Scalar >,
        Right : X2Interface< Scalar = Scalar > + Copy,
      {
        type Output = < X2< Scalar > as $Op::< X2< Scalar > > >::Output;
        #[ inline ]
        fn $op( self, right : &Right ) -> Self::Output
        {
          $Op::< Right >::$op( *self, *right )
        }
      }

    };

  }

  #[ allow( unused_imports ) ]
  pub use impl_x2_rented_op1;
  #[ allow( unused_imports ) ]
  pub use impl_x2_rented_op2;

}

//

crate::mod_interface!
{
  #[ allow( unused_imports )]
  exposed use
  {
    impl_x2_rented_op1,
    impl_x2_rented_op2,
  };
}
