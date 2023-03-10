use darling::ToTokens;
use quote::quote;
use syn::{Expr, Ident, Type};

use crate::derive_input::field::ModelFieldDefine;

pub(crate) enum SelectCols<'s> {
    Col(
        // field name
        &'s Ident,
        // entity
        &'s Type,
    ),
    ColAs(
        // from col
        &'s Expr, 
        // as field name
        &'s Ident),
}

impl<'s> SelectCols<'s> {
    pub fn from(src:&'s ModelFieldDefine,entity:&'s Type)->Self{
        if let Some(expr) = &src.from_col{
            Self::ColAs(expr    , src.ident.as_ref().unwrap())
        }
        else {
            Self::Col(src.ident.as_ref().unwrap(), entity)
        }
    }
}

impl<'s> ToTokens for SelectCols<'s> {
    fn to_tokens(&self, tokens: &mut syn::__private::TokenStream2) {
        let token = match self {
            SelectCols::Col(field, entity) => {
                let col_name  = heck::AsUpperCamelCase(&field.to_string()).to_string();
                let col_value = quote!( <#entity as EntityTrait>::Column:: #col_name );
                quote!(.column(#col_value))
            },
            SelectCols::ColAs(from_col, as_field) => {
                let s = as_field.to_string();
                quote!(.column_as(#from_col, #s))
            },
        };

        tokens.extend(token)

    }
}
