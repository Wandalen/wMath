//!
//! X4 interfaces.
//!

/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  const LENGTH : usize = 4;

  ///
  /// Nominal interface of vector X4.
  ///

  #[ allow( non_camel_case_types ) ]
  pub trait X4NominalInterface
  {

    /// Type of element.
    type Scalar : ScalarInterface;

    /// First element.
    fn _0( &self ) -> Self::Scalar;
    /// Second element.
    fn _1( &self ) -> Self::Scalar;
    /// Third element.
    fn _2( &self ) -> Self::Scalar;
    /// Fourth element.
    fn _3( &self ) -> Self::Scalar;
    /// First element.
    #[ inline ]
    fn x( &self ) -> Self::Scalar
    {
      self._0()
    }
    /// Second element.
    #[ inline ]
    fn y( &self ) -> Self::Scalar
    {
      self._1()
    }
    /// Third element.
    #[ inline ]
    fn z( &self ) -> Self::Scalar
    {
      self._2()
    }
    /// Fourth element.
    #[ inline ]
    fn w( &self ) -> Self::Scalar
    {
      self._3()
    }

    /// Clone as tuple.
    #[ inline ]
    fn clone_as_tuple( &self ) -> ( Self::Scalar , Self::Scalar, Self::Scalar , Self::Scalar )
    {
      ( self._0(), self._1(), self._2(), self._3() )
    }
    /// Clone as array.
    #[ inline ]
    fn clone_as_array( &self ) -> [ Self::Scalar ; LENGTH ]
    {
      [ self._0(), self._1(), self._2(), self._3() ]
    }
    /// Clone as canonical.
    #[ inline ]
    fn clone_as_canonical( &self ) -> X4< Self::Scalar >
    {
      X4::< Self::Scalar >( self._0(), self._1(), self._2(), self._3() )
    }

  }

  ///
  /// Standard interface of vector X4. Implements nominal interface, extending it by constructor `make`.
  ///

  #[ allow( non_camel_case_types ) ]
  pub trait X4BasicInterface : X4NominalInterface
  {

    /// Constructor.
    fn make( _0 : Self::Scalar, _1 : Self::Scalar, _2 : Self::Scalar, _3 : Self::Scalar ) -> Self;

    /// Make an instance filling fields with NaN.
    #[ inline ]
    fn make_nan() -> Self
      where
        Self : Sized,
    {
      use crate::NanLikeInterface;
      Self::make
        (
          Self::Scalar::make_nan_like(),
          Self::Scalar::make_nan_like(),
          Self::Scalar::make_nan_like(),
          Self::Scalar::make_nan_like(),
        )
    }

    /// Make an instance filling fields with default values.
    #[ inline ]
    fn make_default() -> Self
      where
        Self : Sized,
    {
      Self::make
        (
        Self::Scalar::default(),
        Self::Scalar::default(),
        Self::Scalar::default(),
        Self::Scalar::default(),
      )
    }

  }

  ///
  /// Interface of vector X4 for structures with the canonical layout.
  ///

  #[ allow( non_camel_case_types ) ]
  pub trait X4CanonicalInterface : X4BasicInterface + Sized
  {
    /// Assign value.
    #[ inline ]
    fn assign< Src : X4BasicInterface< Scalar = Self::Scalar > >( &mut self, src : Src )
    {
      *self._0_mut() = src._0();
      *self._1_mut() = src._1();
      *self._2_mut() = src._2();
      *self._3_mut() = src._3();
    }

    /// First element.
    fn _0_ref( &self ) -> &Self::Scalar;
    /// Second element.
    fn _1_ref( &self ) -> &Self::Scalar;
    /// Third element.
    fn _2_ref( &self ) -> &Self::Scalar;
    /// Fourth element.
    fn _3_ref( &self ) -> &Self::Scalar;
    /// First element.
    #[ inline ]
    fn x_ref( &self ) -> &Self::Scalar
    {
      self._0_ref()
    }
    /// Second element.
    #[ inline ]
    fn y_ref( &self ) -> &Self::Scalar
    {
      self._1_ref()
    }
    /// Third element.
    #[ inline ]
    fn z_ref( &self ) -> &Self::Scalar
    {
      self._2_ref()
    }
    /// Fourth element.
    #[ inline ]
    fn w_ref( &self ) -> &Self::Scalar
    {
      self._3_ref()
    }

    /// First element.
    fn _0_mut( &mut self ) -> &mut Self::Scalar;
    /// Second element.
    fn _1_mut( &mut self ) -> &mut Self::Scalar;
    /// Third element.
    fn _2_mut( &mut self ) -> &mut Self::Scalar;
    /// Fourth element.
    fn _3_mut( &mut self ) -> &mut Self::Scalar;
    /// First element.
    #[ inline ]
    fn x_mut( &mut self ) -> &mut Self::Scalar
    {
      self._0_mut()
    }
    /// Second element.
    #[ inline ]
    fn y_mut( &mut self ) -> &mut Self::Scalar
    {
      self._1_mut()
    }
    /// Third element.
    #[ inline ]
    fn z_mut( &mut self ) -> &mut Self::Scalar
    {
      self._2_mut()
    }
    /// Fourth element.
    #[ inline ]
    fn w_mut( &mut self ) -> &mut Self::Scalar
    {
      self._3_mut()
    }


    /// Interpret as tuple.
    #[ inline ]
    fn as_tuple( &self ) -> &( Self::Scalar , Self::Scalar, Self::Scalar, Self::Scalar )
    {
      debug_assert_eq!
      (
        core::mem::size_of::< Self >(),
        core::mem::size_of::< ( Self::Scalar , Self::Scalar, Self::Scalar, Self::Scalar ) >()
      );
      unsafe
        {
          std::mem::transmute::< _, _ >( self.as_canonical() )
        }
    }
    /// Interpret as array.
    #[ inline ]
    fn as_array( &self ) -> &[ Self::Scalar ; LENGTH ]
    {
      unsafe
        {
          std::mem::transmute::< _, _ >( self.as_canonical() )
        }
    }
    /// Interpret as slice.
    #[ inline ]
    fn as_slice( &self ) -> &[ Self::Scalar ]
    {
      &self.as_array()[ .. ]
    }
    /// Canonical representation of the vector.
    fn as_canonical( &self ) -> &X4< Self::Scalar >;

    /// Interpret as mutable tuple.
    #[ inline ]
    fn as_tuple_mut( &mut self ) -> &mut ( Self::Scalar , Self::Scalar, Self::Scalar, Self::Scalar )
    {
      debug_assert_eq!
      (
        core::mem::size_of::< Self >(),
        core::mem::size_of::< ( Self::Scalar , Self::Scalar, Self::Scalar, Self::Scalar ) >()
      );
      unsafe
        {
          std::mem::transmute::< _, _ >( self.as_canonical_mut() )
        }
    }
    /// Interpret as mutable array.
    #[ inline ]
    fn as_array_mut( &mut self ) -> &mut [ Self::Scalar ; LENGTH ]
    {
      unsafe
        {
          std::mem::transmute::< _, _ >( self.as_canonical_mut() )
        }
    }
    /// Interpret as mutable slice.
    #[ inline ]
    fn as_slice_mut( &mut self ) -> &mut [ Self::Scalar ]
    {
      &mut self.as_array_mut()[ .. ]
    }
    /// Mutable canonical representation of the vector.
    fn as_canonical_mut( &mut self ) -> &mut X4< Self::Scalar >;

  }

  //

  // NOTE: Conflicting implementation
  // impl< Scalar, Original, Target > crate::From2< Original >
  // for Target
  //   where
  //     Scalar : ScalarInterface,
  //     Original : X4NominalInterface< Scalar = Scalar >,
  //     Target : X4BasicInterface< Scalar = Scalar >,
  // {
  //   #[ inline ]
  //   fn from2( original : Original ) -> Self
  //   {
  //     Self::make( original._0(), original._1(), original._2(), original._3() )
  //   }
  // }
}

//

crate::mod_interface!
{
  prelude use
  {
    X4NominalInterface,
    X4NominalInterface as X4Interface,
    X4BasicInterface,
    X4CanonicalInterface,
  };
}