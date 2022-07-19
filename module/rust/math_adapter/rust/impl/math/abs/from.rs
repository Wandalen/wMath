//!
//! Local implementation of traits From / Into. Required because of limmitations of Rust.
//!

/// Internal namespace.
pub( crate ) mod private
{

  // pub use ::math_common::{ From2, Into2 };

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

crate::mod_interface!
{
  prelude use
  {
    From2,
    Into2,
  };
}
