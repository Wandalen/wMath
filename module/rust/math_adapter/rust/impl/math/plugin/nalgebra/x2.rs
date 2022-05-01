macro_rules! impl_x2_for
{
  ( $Struct : path ) =>
  {

    impl< Scalar > X2Interface for $Struct
    where
      Scalar : ScalarInterface,
    {
      type Scalar = Scalar;

      #[ inline ]
      fn make( _0 : Self::Scalar, _1 : Self::Scalar ) -> Self
      {
        Self::new( _0, _1 )
      }

      #[ inline ]
      fn _0( &self ) -> Self::Scalar
      {
        self.x()
      }

      #[ inline ]
      fn _1( &self ) -> Self::Scalar
      {
        self.y()
      }

      /* */

      #[ inline ]
      fn _0_ref( &self ) -> &Self::Scalar
      {
        &self.x
      }

      #[ inline ]
      fn _1_ref( &self ) -> &Self::Scalar
      {
        &self.y
      }

      /* */

      #[ inline ]
      fn _0_mut( &mut self ) -> &mut Self::Scalar
      {
        &mut self.x
      }

      #[ inline ]
      fn _1_mut( &mut self ) -> &mut Self::Scalar
      {
        &mut self.y
      }

      /* */

    }

    impl< Scalar > X2CanonicalInterface for $Struct
    where
      Scalar : ScalarInterface,
    {

      #[ inline ]
      fn as_canonical( &self ) -> &X2< Self::Scalar >
      {
        debug_assert_eq!( size_of::< Self >(), size_of::< X2< Self::Scalar > >() );
        unsafe
        {
          std::mem::transmute::< _, _ >( self )
        }
      }

      #[ inline ]
      fn as_canonical_mut( &mut self ) -> &mut X2< Self::Scalar >
      {
        debug_assert_eq!( size_of::< Self >(), size_of::< X2< Self::Scalar > >() );
        unsafe
        {
          std::mem::transmute::< _, _ >( self )
        }
      }

    }

    //

    impl< Scalar, Any > crate::AsNalgebraInterface< $Struct > for Any
    where
      Scalar : ScalarInterface,
      Any : X2CanonicalInterface< Scalar = Scalar >,
    {

      fn as_nalgebra( &self ) -> &$Struct
      {
        unsafe
        {
          std::mem::transmute::< _, _ >( self )
        }
      }

      fn as_nalgebra_mut( &mut self ) -> &mut $Struct
      {
        unsafe
        {
          std::mem::transmute::< _, _ >( self )
        }
      }

      // fn clone_as_nalgebra( &self ) -> $Struct
      // {
      //   $Struct::from( self )
      // }

    }

  };

}