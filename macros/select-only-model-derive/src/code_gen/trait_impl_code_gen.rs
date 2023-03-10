use darling::ToTokens;
use quote::quote;
use syn::Ident;

use super::field_conv::SelectCols;

pub(crate) struct TraitImpl<'s> {
    pub(crate) ident: &'s Ident,
    pub(crate) fields: Vec<SelectCols<'s>>,
}

impl<'s> ToTokens for TraitImpl<'s> {
    fn to_tokens(&self, tokens: &mut syn::__private::TokenStream2) {
        let TraitImpl { ident, fields } = self;
        let iter = fields.iter();
        let token = quote! {

            impl ::sql_connection::SelectOnlyModel for #ident{
                fn select_cols<E: EntityTrait>(selector: Select<E>) -> Select<E>{
                    use ::sql_connection::EntityTrait;
                    selector
                    #(#iter)*

                }
            }
        };

        tokens.extend(token)
    }
}
