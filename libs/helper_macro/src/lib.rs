extern crate proc_macro;

use proc_macro2::{TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::ItemStruct;

use proc_macro::TokenStream as CompilerTokenStream;

#[proc_macro_attribute]
pub fn entity(_: CompilerTokenStream, macro_arg_struct: CompilerTokenStream) -> CompilerTokenStream {
    let mut output = TokenStream2::new();
    output.extend( TokenStream2::from(macro_arg_struct));
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