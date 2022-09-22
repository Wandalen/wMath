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
      nominal!( X2NominalInterface, $_type, _0 _1, [ $first ] [ $second ] );

      base!( X2BasicInterface, $_type, $make, _0 _1 );

      canonical!( X2CanonicalInterface, $_type, $as_canonical, _0_ref _1_ref, _0_mut _1_mut, [ 0 ] [ 1 ] );
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
      nominal!( X2NominalInterface, $_type, _0 _1, .$first .$second );

      base!( X2BasicInterface, $_type, $make, _0 _1 );

      canonical!( X2CanonicalInterface, $_type, $as_canonical, _0_ref _1_ref, _0_mut _1_mut, .0 .1 );
    };

    //

    ( $_type:ty, [ $first:literal ], [ $second:literal ] )
    =>
    {
      nominal!( X2NominalInterface, $_type, _0 _1, [ $first ] [ $second ] );
    }
  }

  #[ macro_export ]
  macro_rules! nominal
  {
    ( $interface:ident, $_type:ty, $( $name:ident )*, $( .$getter:tt )* )
    =>
    {
      impl< Scalar > $interface for $_type
      where
        Scalar : ScalarInterface,
      {
        type Scalar = Scalar;

        $(
          #[ inline ]
          fn $name( &self ) -> Self::Scalar
          {
            self. $getter
          }
        )*
      }
    };

    //

    ( $interface:ident, $_type:ty, $( $name:ident )*, $( [ $getter:literal ] )* )
    =>
    {
      impl< Scalar > $interface for $_type
      where
        Scalar : ScalarInterface,
      {
        type Scalar = Scalar;

        $(
          #[ inline ]
          fn $name( &self ) -> Self::Scalar
          {
            self[ $getter ]
          }
        )*
      }
    };
  }

  #[ macro_export ]
  macro_rules! base
  {
    ( $interface:ident, $_type:ty, $make:expr, $( $name:ident )* )
    =>
    {
      impl< Scalar > $interface for $_type
      where
        Scalar : ScalarInterface,
      {
        #[ inline ]
        fn make( $( $name : Self::Scalar, )* ) -> Self
        {
          $make( $( $name, )* )
        }
      }
    }
  }

  #[ macro_export ]
  macro_rules! canonical
  {
    ( $interface:ident, $_type:ty, $as_canonical:expr, $( $ref_name:ident )*, $( $mut_name:ident )*, $( .$getter:tt )* )
    =>
    {
      impl< Scalar > $interface for $_type
      where
        Scalar : ScalarInterface,
      {
        $(
          #[ inline ]
          fn $ref_name( &self ) -> &Self::Scalar
          {
            &self. $getter
          }

          #[ inline ]
          fn $mut_name( &mut self ) -> &mut Self::Scalar
          {
            &mut self. $getter
          }
        )*

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
      }
    };

    //

    ( $interface:ident, $_type:ty, $as_canonical:expr, $( $ref_name:ident )*, $( $mut_name:ident )*, $( [ $getter:tt ] )* )
    =>
    {
      impl< Scalar > $interface for $_type
      where
        Scalar : ScalarInterface,
      {
        $(
          #[ inline ]
          fn $ref_name( &self ) -> &Self::Scalar
          {
            &self[ $getter ]
          }

          #[ inline ]
          fn $mut_name( &mut self ) -> &mut Self::Scalar
          {
            &mut self[ $getter ]
          }
        )*

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