macro_rules! impl_x2_for
{
  ( $Struct : path ) =>
  {

    impl< Scalar > x2_interface for $Struct
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
        self.width
      }

      #[ inline ]
      fn _1( &self ) -> Self::Scalar
      {
        self.height
      }

      /* */

    }

    impl< Scalar > x2_canonical_interface for $Struct
    where
      Scalar : ScalarInterface,
    {

      /* */

      #[ inline ]
      fn _0_ref( &self ) -> &Self::Scalar
      {
        &self.width
      }

      #[ inline ]
      fn _1_ref( &self ) -> &Self::Scalar
      {
        &self.height
      }

      /* */

      #[ inline ]
      fn _0_mut( &mut self ) -> &mut Self::Scalar
      {
        &mut self.width
      }

      #[ inline ]
      fn _1_mut( &mut self ) -> &mut Self::Scalar
      {
        &mut self.height
      }

      /* */

      #[ inline ]
      fn as_canonical( &self ) -> &x2< Self::Scalar >
      {
        debug_assert_eq!( size_of::< Self >(), size_of::< x2< Self::Scalar > >() );
        unsafe
        {
          std::mem::transmute::< _, _ >( self )
        }
      }

      #[ inline ]
      fn as_canonical_mut( &mut self ) -> &mut x2< Self::Scalar >
      {
        debug_assert_eq!( size_of::< Self >(), size_of::< x2< Self::Scalar > >() );
        unsafe
        {
          std::mem::transmute::< _, _ >( self )
        }
      }

    }

  };

}