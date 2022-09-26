use proc_macro::TokenStream;
use proc_macro_tools::
{
  parse_macro_input,
  ParseStream,
  proc_macro2,
  syn
};
use proc_macro_tools::quote::
{
  format_ident,
  quote
};
use proc_macro_tools::syn::Fields;
use proc_macro_tools::syn::parse::Parse;
use crate::proc_macro2::{Literal, TokenTree};
use crate::syn::
{
  Ident,
  Type
};

struct Input
{
  ident : Ident,
  identifiers : Vec< TokenTree >,
  with_generic : bool,
  named : bool,
  element_type : Type,
}

impl Parse for Input
{
  fn parse( input : ParseStream ) -> syn::Result< Self >
  {
    let item : syn::ItemStruct = input.parse()?;
    let ident = item.ident;
    let with_generic = !item.generics.params.is_empty();
    let named;
    let element_type;
    let identifiers : Vec< TokenTree > = match item.fields
    {
      Fields::Named( fields ) =>
        {
          named = true;
          element_type = fields.named.first().unwrap().ty.clone();
          fields.named.iter().map( | field | TokenTree::Ident( field.ident.clone().unwrap() ) ).collect()
        },
      Fields::Unnamed( fields ) =>
        {
          named = false;
          element_type = fields.unnamed.first().unwrap().ty.clone();
          ( 0..fields.unnamed.len() ).map( | idx | TokenTree::Literal( Literal::usize_unsuffixed( idx ) ) ).collect()
        },
      _ => return Err( input.error( "Expected struct with fields" ) ),
    };

    Ok( Input { ident, identifiers, with_generic, named, element_type } )
  }
}

#[ proc_macro_derive( VectorInterfaces ) ]
pub fn derive_vector_interfaces( input : TokenStream ) -> TokenStream
{
  let input = parse_macro_input!( input as Input );

  let dimension_count = input.identifiers.len();
  let nominal_interface_name = format_ident!( "X{}NominalInterface", dimension_count );
  let basic_interface_name = format_ident!( "X{}BasicInterface", dimension_count );
  let canonical_interface_name = format_ident!( "X{}CanonicalInterface", dimension_count );
  let canonical_type = format_ident!( "X{}", dimension_count );

  let nominal = impl_nominal( nominal_interface_name, &input );
  let basic = impl_basic( basic_interface_name, &input );
  let canonical = impl_canonical( canonical_interface_name, canonical_type, &input );

  let output = quote!
  (
    #nominal

    #basic

    #canonical
  );
  output.into()
}

fn impl_nominal( interface_name : Ident, input : &Input ) -> proc_macro2::TokenStream
{
  let Input { ident, identifiers, with_generic, named : _, element_type } = input;
  let function_names : Vec< Ident > = ( 0..identifiers.len() ).into_iter().map( | idx |  format_ident!( "_{}", idx ) ).collect();
  let methods = quote!
  (
    #(
      #[ inline ]
      fn #function_names( &self ) -> Self::Scalar
      {
        self.#identifiers
      }
    )*
  );

  if *with_generic
  {
    quote!
    (
      impl< #element_type > #interface_name for #ident< #element_type >
      where
        #element_type : ScalarInterface,
      {
        type Scalar = #element_type;

        #methods

      }
    )
  }
  else
  {
    quote!
    (
      impl #interface_name for #ident
      {
        type Scalar = #element_type;

        #methods

      }
    )
  }
}

fn impl_basic( interface_name : Ident, input : &Input ) -> proc_macro2::TokenStream
{
  let Input { ident, identifiers, with_generic, named, element_type } = input;
  let names : Vec< Ident > = ( 0..identifiers.len() ).into_iter().map( | idx |  format_ident!( "_{}", idx ) ).collect();

  let constructor = if *named
  {
    quote!
    (
      #[ inline ]
      fn make( #( #identifiers : Self::Scalar, )* ) -> Self
      {
        Self { #( #identifiers, )* }
      }
    )
  }
  else
  {
    quote!
    (
      #[ inline ]
      fn make( #( #names : Self::Scalar, )* ) -> Self
      {
        Self ( #( #names, )* )
      }
    )
  };

  if *with_generic
  {
    quote!
    (
      impl< #element_type > #interface_name for #ident< #element_type >
      where
        #element_type : ScalarInterface,
      {
        #constructor
      }
    )
  }
  else
  {
    quote!
    (
      impl #interface_name for #ident
      {
        #constructor
      }
    )
  }
}

fn impl_canonical( interface_name : Ident, canonical_type : Ident, input : &Input ) -> proc_macro2::TokenStream
{
  let Input { ident, identifiers, with_generic, named : _, element_type } = input;
  let ref_names : Vec< Ident > = ( 0..identifiers.len() ).into_iter().map( | idx | format_ident!( "_{}_ref", idx ) ).collect();
  let mut_names : Vec< Ident > = ( 0..identifiers.len() ).into_iter().map( | idx | format_ident!( "_{}_mut", idx ) ).collect();
  let methods = quote!
  (
    #(
      #[ inline ]
      fn #ref_names( &self ) -> &Self::Scalar
      {
        &self.#identifiers
      }

      #[ inline ]
      fn #mut_names( &mut self ) -> &mut Self::Scalar
      {
        &mut self.#identifiers
      }
    )*

    #[ inline ]
    fn as_canonical( &self ) -> &#canonical_type< #element_type >
    {
      unsafe { std::mem::transmute::< _, _ >( self ) }
    }

    #[ inline ]
    fn as_canonical_mut( &mut self ) -> &mut #canonical_type< #element_type >
    {
      unsafe { std::mem::transmute::< _, _ >( self ) }
    }
  );

  if *with_generic
  {
    quote!
    (
      impl< #element_type > #interface_name for #ident< #element_type >
      where
        #element_type : ScalarInterface,
      {
        #methods
      }
    )
  }
  else
  {
    quote!
    (
      impl #interface_name for #ident
      {
        #methods
      }
    )
  }
}