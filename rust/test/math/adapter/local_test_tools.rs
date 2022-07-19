
///
/// Required to convert integets to floats.
///

#[ macro_export ]
macro_rules! num
{

  () =>
  {
  };

  ( $num : expr ) =>
  {
    num_traits::cast::< _, T >( $num ).unwrap()
  };

  ( $( $num : expr ),+ ) =>
  {(
    $( num_traits::cast::< _, T >( $num ).unwrap() ),+
  )};

}
