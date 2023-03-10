use darling::ToTokens;
use heck::ToUpperCamelCase;
use quote::{format_ident, quote};
use syn::{Ident, Type};

use crate::derive_input::field::ModelFieldDefine;

pub(crate) enum SelectCols<'s> {
    Col(
        // field name
        &'s Ident,
        // entity
        &'s Type,
    ),
    ColAs(
        &'s Type,
        // from col
        &'s Ident,
        // as field name
        &'s Ident,
    ),
}

impl<'s> SelectCols<'s> {
    pub fn from(src: &'s ModelFieldDefine, entity: &'s Type) -> Self {
        if let Some(expr) = &src.from_col {
            Self::ColAs(entity, expr, src.ident.as_ref().unwrap())
        } else {
            Self::Col(src.ident.as_ref().unwrap(), entity)
        }
    }

    fn get_entity(&self) -> &'s Type {
        match self {
            SelectCols::Col(_, t) | SelectCols::ColAs(t, _, _) => t,
        }
    }

    fn get_target_col(&self) -> Ident {
        let v = match self {
            SelectCols::Col(f, _) | SelectCols::ColAs(_, f, _) => f,
        }
        .to_string()
        .to_upper_camel_case();
        format_ident!("{v}")
    }
}

impl<'s> ToTokens for SelectCols<'s> {
    fn to_tokens(&self, tokens: &mut syn::__private::TokenStream2) {
        let entity = self.get_entity();
        let origin_col = self.get_target_col();
        let col_value = quote!(
            <#entity as ::sql_connection::EntityTrait>::Column :: #origin_col
        );
        let token = match self {
            SelectCols::Col(..) => {
                quote!(let selector = ::sql_connection::QuerySelect::column(selector, #col_value);)
            }
            SelectCols::ColAs(_, as_field, ..) => {
                let s = as_field.to_string();
                quote!(let selector = ::sql_connection::QuerySelect::column_as(selector, #col_value, #s);)
            }
        };

        tokens.extend(token)
    }
}
