macro_rules! impl_x2_for
{

  ( $Struct1 : ident $( :: $Struct2 : ident )* < $Params : ident > ) =>
  {

    impl< Scalar > X2Interface for $Struct1 $( :: $Struct2 )* < $Params >
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
        self.x
      }

      #[ inline ]
      fn _1( &self ) -> Self::Scalar
      {
        self.y
      }

      /* */

    }

    impl< Scalar > X2CanonicalInterface for $Struct1 $( :: $Struct2 )* < $Params >
    where
      Scalar : ScalarInterface,
    {

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

    impl< Scalar, Any > crate::AsCgmathNonCanonicalInterface< $Struct1 $( :: $Struct2 )* < $Params > > for Any
    where
      Scalar : ScalarInterface,
      Any : X2Interface< Scalar = Scalar > + Copy,
    {

      fn clone_as_cgmath( &self ) -> $Struct1 $( :: $Struct2 )* < $Params > /* xxx : cover */
      {
        $Struct1 $( :: $Struct2 )* :: < $Params > :: from2( *self )
      }

    }

    //

    impl< Scalar, Any > crate::AsCgmathCanonicalInterface< $Struct1 $( :: $Struct2 )* < $Params > > for Any
    where
      Scalar : ScalarInterface,
      Any : X2CanonicalInterface< Scalar = Scalar >,
    {

      fn as_cgmath( &self ) -> &$Struct1 $( :: $Struct2 )* < $Params > /* xxx : cover */
      {
        unsafe
        {
          std::mem::transmute::< _, _ >( self )
        }
      }

      fn as_cgmath_mut( &mut self ) -> &mut $Struct1 $( :: $Struct2 )* < $Params > /* xxx : cover */
      {
        unsafe
        {
          std::mem::transmute::< _, _ >( self )
        }
      }

    }

    //

    // trait x2_interface2
    // {
    //   type Scalar;
    // }

    // impl< Scalar, X2 > From< X2< Scalar = Scalar > > for $Struct1 $( :: $Struct2 )* < $Params >
    // where
    //   Scalar : ScalarInterface,
    //   X2 : X2Interface< Scalar = Scalar >,
    // {
    //   fn from( src : $Struct1 $( :: $Struct2 )* < $Params > ) -> Self
    //   {
    //     Self::make( src._0(), src._1() )
    //   }
    // }

  };

}