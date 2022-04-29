/// Internal namespace.
pub mod internal
{

  ///
  /// Local implementation of trait From.
  ///
  /// Separate implementation is required because of restriction : "only traits defined in the current crate can be implemented for a type parameter".
  ///

  pub trait From2< T > : Sized
  {
    /// Performs the conversion.
    fn from2( _ : T ) -> Self;
  }

  // impl< Target > From2< &Target >
  // for Target
  // {
  //   fn from2( original : &Target ) -> Self
  //   {
  //     < Self as From2< Target > >::from2( *original )
  //   }
  // }

  ///
  /// Local implementation of trait Into.
  ///
  /// Separate implementation is required because of restriction : "only traits defined in the current crate can be implemented for a type parameter".
  ///

  pub trait Into2< T > : Sized
  {
    /// Performs the conversion.
    fn into2( self ) -> T;
  }

  impl< Target, Original > crate::Into2< Target > for Original
  where
    Target : crate::From2< Original >,
  {
    fn into2( self ) -> Target
    {
      Target::from2( self )
    }
  }

}

/// Exposed namespace of the module.
pub mod exposed
{
  use super::internal as i;
  pub use i::From2;
  pub use i::Into2;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  use super::internal as i;
  pub use i::From2;
  pub use i::Into2;
}
