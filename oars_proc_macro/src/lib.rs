//! This crate provides procedural macros for `oars`

extern crate proc_macro;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

/// Create a copy of the struct suffixed by "Checked"
///
/// This macro exists to provide a convenient way to derive the checked variants of constructors
/// for oars. In oars, the pattern for creating constructors with checked parameters involves
/// creating an identical struct, usually suffixed by "Checked" (so the checked variant of `Bose`
/// is `BoseChecked`). This macro automatically creates an identical struct but with a different
/// name, for convenience.
#[proc_macro_derive(Checked)]
pub fn derive_checked_ctor(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = parse_macro_input!(item as DeriveInput);

    if let Data::Struct(_existing_struct) = &input.data {
        let mut new_struct: DeriveInput = input.clone();
        let new_struct_name = format!("{}{}", new_struct.ident, "Checked");
        new_struct.ident = Ident::new(&new_struct_name, Span::call_site());
        let expanded = quote!(#new_struct);
        proc_macro::TokenStream::from(expanded)
    } else {
        panic!("Expected struct")
    }
}
