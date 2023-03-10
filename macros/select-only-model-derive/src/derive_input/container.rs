use darling::{ast::Data, util::Ignored, FromDeriveInput};
use syn::Generics;

use super::field::ModelFieldDefine;

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(select_only),
    supports(struct_named),
    and_then = "Self::check"
)]
pub struct DeriveContainer {
    pub(crate) ident: syn::Ident,
    generics: Generics,
    /// original model
    pub(crate) origin: syn::Type,
    pub(crate) data: Data<Ignored, ModelFieldDefine>,
}

pub struct DeriveInput {
    pub(crate) ident: syn::Ident,
    pub(crate) origin: syn::Type,
    pub(crate) fields: Vec<ModelFieldDefine>,
}

impl DeriveContainer {
    fn check(self) -> darling::Result<Self> {
        if self.generics.type_params().count() != 0
            || self.generics.lifetimes().count() != 0
            || self.generics.const_params().count() != 0
        {
            Err(darling::Error::unexpected_type("Generics"))?;
        }
        Ok(self)
    }

    pub(crate) fn map(self) -> DeriveInput {
        let DeriveContainer {
            ident,
            generics: _,
            origin,
            data,
        } = self;
        DeriveInput {
            ident,
            origin,
            fields: data.take_struct().unwrap().fields,
        }
    }
}
