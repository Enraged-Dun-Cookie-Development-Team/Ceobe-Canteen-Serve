mod codegen;
#[macro_use]
mod utils;
mod checker_info;
mod inner_checker_info;
use inner_checker_info::InnerCheckerInfo;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn check_obj(params: TokenStream, item: TokenStream) -> TokenStream {
    let params = parse_macro_input!(params as checker_info::CheckerInfo);
    let body = parse_macro_input!(item as ItemStruct);

    let body = syn_error!(InnerCheckerInfo::from_item_struct(body));


    unimplemented!()
}
