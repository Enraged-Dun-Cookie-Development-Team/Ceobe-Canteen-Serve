use code_gen::to_code_gen;
use darling::FromDeriveInput;
use derive_input::container::DeriveContainer;
use syn::{parse_macro_input, DeriveInput};

mod code_gen;
mod derive_input;

#[proc_macro_derive(SelectOnlyModel, attributes(select_only))]
pub fn select_only_model_derive(
    derive_input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(derive_input as DeriveInput);
    let ret = 'code_gen: {
        let input = match DeriveContainer::from_derive_input(&derive_input) {
            Ok(input) => input,
            Err(err) => break 'code_gen Err(err),
        }
        .map();
        let code_gen = to_code_gen(&input);

        Ok(quote::quote!(#code_gen))
    };

    match ret {
        Ok(ts) => ts,
        Err(err) => err.write_errors(),
    }
    .into()
}
