/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;


  ///
  /// Implement dereferencing of regular math object to math objects of math lib of choice.
  ///

  // xxx : rename
  #[ allow( unused_macros ) ]
  #[ macro_export ]
  macro_rules! impl_x2_deref
  {

    () => {};

    ( $Va : ident $( :: $Vb : ident )* ) =>
    {

      impl< Scalar > Deref for X2< Scalar >
      where
        Scalar : ScalarInterface,
      {
        type Target = $Va $( :: $Vb )*< Scalar >;
        fn deref( &self ) -> &Self::Target
        {
          self.as_foreign()
        }
      }

      impl< Scalar > DerefMut for X2< Scalar >
      where
        Scalar : ScalarInterface,
      {
        fn deref_mut( &mut self ) -> &mut Self::Target
        {
          self.as_foreign_mut()
        }
      }

    };

  }

  #[ allow( unused_imports ) ]
  pub( crate ) use impl_x2_deref;

}

//

crate::mod_interface!
{
  exposed( crate ) use impl_x2_deref;
}
