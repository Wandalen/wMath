//!
//! Macro to implement traits in the x2_interface module.
//!

/// Internal namespace.
pub ( crate ) mod private
{
  use crate::*;

  ///
  /// Implement traits in the x2_interface module for an arbitrary type.
  /// Arguments :
  /// Type with the 'Scalar' generic;
  /// Getter for x (the index syntax (e.g. '[ value ]') and the dot syntax (e.g. '.value') are supported );
  /// Getter for y (the index syntax (e.g. '[ value ]') and the dot syntax (e.g. '.value') are supported );
  /// Closure, which takes two scalars and returns a new instance of the Type;
  /// Closure ,which takes the ref to the current instance and returns the &X2< Scalar > representation;
  /// If only the first three arguments are provided, only X2BasicInterface will be implemented.
  ///

  #[ macro_export ]
  macro_rules! impl_x2_interfaces
  {

    () => {};

    (
      $_type:ty,
      [ $first:literal ],
      [ $second:literal ],
      $make:expr,
      $as_canonical:expr
    )
    =>
    {
      impl_x2_interfaces!( $_type, [ $first ], [ $second ] );

      //

      impl< Scalar > X2BasicInterface for $_type
      where
        Scalar : ScalarInterface,
      {
        #[ inline ]
        fn make( _0 : Self::Scalar, _1 : Self::Scalar ) -> Self
        {
          $make( _0, _1 )
        }
      }

      //

      impl< Scalar > X2CanonicalInterface for $_type
      where
        Scalar : ScalarInterface,
      {

        /* */

        #[ inline ]
        fn _0_ref( &self ) -> &Self::Scalar
        {
          &self[ $first ]
        }

        #[ inline ]
        fn _1_ref( &self ) -> &Self::Scalar
        {
          &self[ $second ]
        }

        /* */

        #[ inline ]
        fn _0_mut( &mut self ) -> &mut Self::Scalar
        {
          &mut self[ $first ]
        }

        #[ inline ]
        fn _1_mut( &mut self ) -> &mut Self::Scalar
        {
          &mut self[ $second ]
        }

        /* */

        #[ inline ]
        fn as_canonical( &self ) -> &X2< Self::Scalar >
        {
          $as_canonical( self )
        }

        #[ inline ]
        fn as_canonical_mut( &mut self ) -> &mut X2< Self::Scalar >
        {
          $as_canonical( self )
        }

        /* */

      }
    };

    ( $_type:ty, [ $first:expr ], [ $second:expr ] )
    =>
    {
      impl< Scalar > X2NominalInterface for $_type
      where
        Scalar : ScalarInterface,
      {
        type Scalar = Scalar;

        #[ inline ]
        fn _0( &self ) -> Self::Scalar
        {
          self[ $first ]
        }

        #[ inline ]
        fn _1( &self ) -> Self::Scalar
        {
          self[ $second ]
        }
      }
    };

    //

    (
      $_type:ty,
      .$first:tt,
      .$second:tt,
      $make:expr,
      $as_canonical:expr
    )
    =>
    {
      impl_x2_interfaces!( $_type, .$first, .$second );

      //

      impl< Scalar > X2BasicInterface for $_type
      where
        Scalar : ScalarInterface,
      {
        #[ inline ]
        fn make( _0 : Self::Scalar, _1 : Self::Scalar ) -> Self
        {
          $make( _0, _1 )
        }
      }

      //

      impl< Scalar > X2CanonicalInterface for $_type
      where
        Scalar : ScalarInterface,
      {

        /* */

        #[ inline ]
        fn _0_ref( &self ) -> &Self::Scalar
        {
          &self. $first
        }

        #[ inline ]
        fn _1_ref( &self ) -> &Self::Scalar
        {
          &self. $second
        }

        /* */

        #[ inline ]
        fn _0_mut( &mut self ) -> &mut Self::Scalar
        {
          &mut self. $first
        }

        #[ inline ]
        fn _1_mut( &mut self ) -> &mut Self::Scalar
        {
          &mut self. $second
        }

        /* */

        #[ inline ]
        fn as_canonical( &self ) -> &X2< Self::Scalar >
        {
          $as_canonical( self )
        }

        #[ inline ]
        fn as_canonical_mut( &mut self ) -> &mut X2< Self::Scalar >
        {
          $as_canonical( self )
        }

        /* */

      }
    };

    ( $_type:ty, .$first:tt, .$second:tt )
    =>
    {
      impl< Scalar > X2NominalInterface for $_type
      where
        Scalar : ScalarInterface,
      {
        type Scalar = Scalar;

        #[ inline ]
        fn _0( &self ) -> Self::Scalar
        {
          self. $first
        }

        #[ inline ]
        fn _1( &self ) -> Self::Scalar
        {
          self. $second
        }
      }
    };
  }

  pub use impl_x2_interfaces;

}

//

crate::mod_interface!
{
  exposed use impl_x2_interfaces;
}