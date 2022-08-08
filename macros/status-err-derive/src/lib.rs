#[macro_use]
mod utils;
mod code_gen;
mod input_loading;
use darling::FromDeriveInput;
use input_loading::derive_info::StatusErrorDeriveInfo;
use proc_macro::TokenStream;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput};

#[proc_macro_derive(StatusErr, attributes(status_err))]
pub fn status_error_derive(derive_input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(derive_input as DeriveInput);
    if !derive_input.generics.params.is_empty() {
        return syn::Error::new(
            derive_input.generics.params.span(),
            "Status Error can not using Generic",
        )
        .to_compile_error()
        .into();
    }
    if let Data::Struct(_) | Data::Union(_) = derive_input.data {
        return syn::Error::new(
            derive_input.span(),
            "Status Error can Only using on Enum",
        )
        .to_compile_error()
        .into();
    }

    let status_err = darling_error!(
        StatusErrorDeriveInfo::from_derive_input(&derive_input)
    );

    let token = syn_error!(status_err.checking());

    quote::quote! {
        #token
    }
    .into()
}
