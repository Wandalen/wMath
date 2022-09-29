//!
//! Macro to implement rented operators.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  ///
  /// Declare operation with 1 operand renting its implementation from math lib of choice.
  ///

  #[ allow( unused_macros ) ]
  #[ macro_export ]
  macro_rules! impl_rented_op1
  {

    () => {};

    (
      $Op : ident,
      $op : ident,
      $Va : ident $( :: $Vb : ident )* ,
      $For : ident $(,)?
    )
    =>
    // ( $Op : ident, $op : ident, $Va : ident $( :: $Vb : ident )* ) =>
    {

      impl< Scalar > $Op for $For< Scalar >
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

      impl< Scalar > $Op for &$For< Scalar >
      where
        Scalar : ScalarInterface + $Op< Output = Scalar >,
      {
        type Output = < $For< Scalar > as $Op >::Output;
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
  macro_rules! impl_rented_op2
  {

    () => {};

    (
      $Op : ident,
      $op : ident,
      $Va : ident $( :: $Vb : ident )*,
      $For : ident $(,)?
    )
    =>
    // ( $Op : ident, $op : ident, $Va : ident $( :: $Vb : ident )* ) =>
    {

      $crate::idents_concat!
      {
        impl< Right, Scalar > $Op< Right > for $For< Scalar >
        where
          Scalar : ScalarInterface + $Op< Output = Scalar >,
          Right : [< $For Interface >]< Scalar = Scalar > + Copy,
          // Right : X2Interface< Scalar = Scalar > + Copy,
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
      }

      //

      $crate::idents_concat!
      {
        impl< Right, Scalar > $Op< &Right > for &$For< Scalar >
        where
          Scalar : ScalarInterface + $Op< Output = Scalar >,
          Right : [< $For Interface >]< Scalar = Scalar > + Copy,
          // Right : X2Interface< Scalar = Scalar > + Copy,
        {
          type Output = < $For< Scalar > as $Op::< $For< Scalar > > >::Output;
          #[ inline ]
          fn $op( self, right : &Right ) -> Self::Output
          {
            $Op::< Right >::$op( *self, *right )
          }
        }
      }

    };

  }

  #[ allow( unused_imports ) ]
  pub use impl_rented_op1;
  #[ allow( unused_imports ) ]
  pub use impl_rented_op2;

}

//

crate::mod_interface!
{
  #[ allow( unused_imports )]
  exposed use
  {
    impl_rented_op1,
    impl_rented_op2,
  };
}
