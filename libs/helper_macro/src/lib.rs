extern crate proc_macro;

use proc_macro2::{TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::ItemStruct;

use proc_macro::TokenStream as CompilerTokenStream;

#[proc_macro_attribute]
pub fn entity(_macro_arg_attr: CompilerTokenStream, macro_arg_struct: CompilerTokenStream) -> CompilerTokenStream {
    let token_struct = TokenStream2::from(macro_arg_struct);
    let entity_struct = syn::parse2::<ItemStruct>(token_struct).unwrap();

    let mut output = TokenStream2::new();
    output.extend(entity_struct.to_token_stream());
    output.extend(quote! {

        startup::on_startup! {
            crate::database::model_register::static_register_model(|db_register| {
                let db_register = db_register.register_model(Entity);
                db_register
            });
        }

    });
    output.into()
}