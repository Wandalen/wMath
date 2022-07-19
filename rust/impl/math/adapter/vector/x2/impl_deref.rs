//!
//! Macro to implement deref trait.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  ///
  /// Implement dereferencing of regular math object to math objects of math lib of choice.
  ///

  #[ allow( unused_macros ) ]
  #[ macro_export ]
  macro_rules! impl_vector_deref
  {

    () => {};

    (
      $Va : ident $( :: $Vb : ident )* ,
      $For : ident $(,)?
    )
    =>
    {

      impl< Scalar > Deref for $For< Scalar >
      where
        Scalar : ScalarInterface,
      {
        type Target = $Va $( :: $Vb )*< Scalar >;
        fn deref( &self ) -> &Self::Target
        {
          self.as_foreign()
        }
      }

      impl< Scalar > DerefMut for $For< Scalar >
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
  pub use impl_vector_deref;

}

//

crate::mod_interface!
{
  exposed use impl_vector_deref;
}
