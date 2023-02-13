//!
//! Macro to implement XnNominalInterface, XnBasicInterface, XnCanonicalInterface traits for an arbitrary type.
//!

/// Internal namespace.
pub ( crate ) mod private
{
  use crate::*;

  ///
  /// Implement XnNominalInterface, XnBasicInterface, XnCanonicalInterface traits for an arbitrary type.
  /// Arguments :
  /// Type with the 'Scalar' generic;
  /// Getter for x (the index syntax (e.g. '[ value ]') and the dot syntax (e.g. '.value') are supported );
  /// Getter for y;
  /// Getter for z (optional);
  /// Getter for w (optional);
  /// Closure, which takes scalars and returns a new instance of the type;
  /// Closure, which takes the ref to the current instance and returns the '&Xn< Scalar >' representation;
  /// If the last two arguments are not provided, only XnNominalInterface will be implemented.
  ///

  #[ macro_export ]
  macro_rules! impl_interfaces
  {
    () => {};

    //

    (
      $_type:ty,
      [ $first:literal ],
      [ $second:literal ]
    )
    =>
    {
      impl_vector_nominal_interface!( X2NominalInterface, $_type, _0 _1, [ $first ] [ $second ] );
    };

    //

    (
      $_type:ty,
      [ $first:literal ],
      [ $second:literal ],
      [ $third:literal ]
    )
    =>
    {
      impl_vector_nominal_interface!( X3NominalInterface, $_type, _0 _1 _2, [ $first ] [ $second ] [ $third ] );
    };

    //

    (
      $_type:ty,
      [ $first:literal ],
      [ $second:literal ],
      [ $third:literal ],
      [ $fourth:literal ]
    )
    =>
    {
      impl_vector_nominal_interface!( X4NominalInterface, $_type, _0 _1 _2 _3, [ $first ] [ $second ] [ $third ] [ $fourth ] );
    };

    //

    (
      $_type:ty,
      [ $first:literal ],
      [ $second:literal ],
      $make:expr,
      $as_canonical:expr
    )
    =>
    {
      impl_vector_nominal_interface!( X2NominalInterface, $_type, _0 _1, [ $first ] [ $second ] );

      impl_vector_basic_interface!( X2BasicInterface, $_type, $make, _0 _1 );

      impl_vector_canonical_interface!( X2CanonicalInterface, $_type, X2, $as_canonical, _0_ref _1_ref, _0_mut _1_mut, [ $first ] [ $second ] );
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
      impl_vector_nominal_interface!( X2NominalInterface, $_type, _0 _1, .$first .$second );

      impl_vector_basic_interface!( X2BasicInterface, $_type, $make, _0 _1 );

      impl_vector_canonical_interface!( X2CanonicalInterface, $_type, X2, $as_canonical, _0_ref _1_ref, _0_mut _1_mut, .$first .$second );
    };

    //

    (
      $_type:ty,
      [ $first:literal ],
      [ $second:literal ],
      [ $third:literal ],
      $make:expr,
      $as_canonical:expr
    )
    =>
    {
      impl_vector_nominal_interface!( X3NominalInterface, $_type, _0 _1 _2, [ $first ] [ $second ] [ $third ] );

      impl_vector_basic_interface!( X3BasicInterface, $_type, $make, _0 _1 _2 );

      impl_vector_canonical_interface!
      (
        X3CanonicalInterface,
        $_type,
        X3,
        $as_canonical,
        _0_ref _1_ref _2_ref,
        _0_mut _1_mut _2_mut,
        [ $first ] [ $second ] [ $third ]
      );
    };

    //

    (
      $_type:ty,
      .$first:tt,
      .$second:tt,
      .$third:tt,
      $make:expr,
      $as_canonical:expr
    )
    =>
    {
      impl_vector_nominal_interface!( X3NominalInterface, $_type, _0 _1 _2, .$first .$second .$third );

      impl_vector_basic_interface!( X3BasicInterface, $_type, $make, _0 _1 _2 );

      impl_vector_canonical_interface!
      (
        X3CanonicalInterface,
        $_type,
        X3,
        $as_canonical,
        _0_ref _1_ref _2_ref,
        _0_mut _1_mut _2_mut,
        .$first .$second .$third
      );
    };

    //

    (
      $_type:ty,
      [ $first:literal ],
      [ $second:literal ],
      [ $third:literal ],
      [ $fourth:literal ],
      $make:expr,
      $as_canonical:expr
    )
    =>
    {
      impl_vector_nominal_interface!( X4NominalInterface, $_type, _0 _1 _2 _3, [ $first ] [ $second ] [ $third ] [ $fourth ] );

      impl_vector_basic_interface!( X4BasicInterface, $_type, $make, _0 _1 _2 _3 );

      impl_vector_canonical_interface!
      (
        X4CanonicalInterface,
        $_type,
        X4,
        $as_canonical,
        _0_ref _1_ref _2_ref _3_ref,
        _0_mut _1_mut _2_mut _3_mut,
        [ $first ] [ $second ] [ $third ] [ $fourth ]
      );
    };

    //

    (
      $_type:ty,
      .$first:tt,
      .$second:tt,
      .$third:tt,
      .$fourth:tt,
      $make:expr,
      $as_canonical:expr
    )
    =>
    {
      impl_vector_nominal_interface!( X4NominalInterface, $_type, _0 _1 _2 _3, .$first .$second .$third .$fourth );

      impl_vector_basic_interface!( X4BasicInterface, $_type, $make, _0 _1 _2 _3 );

      impl_vector_canonical_interface!
      (
        X4CanonicalInterface,
        $_type,
        X4,
        $as_canonical,
        _0_ref _1_ref _2_ref _3_ref,
        _0_mut _1_mut _2_mut _3_mut,
        .$first .$second .$third .$fourth
      );
    };
  }

  ///
  /// Implement nominal vector interface for a type.
  ///

  #[ macro_export ]
  macro_rules! impl_vector_nominal_interface
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

  ///
  /// Implement basic vector interface for a type.
  ///

  #[ macro_export ]
  macro_rules! impl_vector_basic_interface
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

  ///
  /// Implement canonical vector interface for a type.
  ///

  #[ macro_export ]
  macro_rules! impl_vector_canonical_interface
  {
    ( $interface:ident, $_type:ty, $canonical_type:ident, $as_canonical:expr, $( $ref_name:ident )*, $( $mut_name:ident )*, $( .$getter:tt )* )
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
        fn as_canonical( &self ) -> &$canonical_type< Self::Scalar >
        {
          $as_canonical( self )
        }

        #[ inline ]
        fn as_canonical_mut( &mut self ) -> &mut $canonical_type< Self::Scalar >
        {
          $as_canonical( self )
        }
      }
    };

    //

    ( $interface:ident, $_type:ty, $canonical_type:ident, $as_canonical:expr, $( $ref_name:ident )*, $( $mut_name:ident )*, $( [ $getter:tt ] )* )
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
        fn as_canonical( &self ) -> & $canonical_type < Self::Scalar >
        {
          $as_canonical( self )
        }

        #[ inline ]
        fn as_canonical_mut( &mut self ) -> &mut $canonical_type < Self::Scalar >
        {
          $as_canonical( self )
        }
      }
    };
  }

  pub use impl_interfaces;
  pub use impl_vector_nominal_interface;
  pub use impl_vector_basic_interface;
  pub use impl_vector_canonical_interface;
}

//

crate::mod_interface!
{
  prelude use impl_interfaces;
  prelude use impl_vector_nominal_interface;
  prelude use impl_vector_basic_interface;
  prelude use impl_vector_canonical_interface;
}