use darling::FromMeta;
use proc_macro2::Ident;
use syn::{Meta, NestedMeta, Visibility};

use super::{pub_vis, utils::load_from_meta_list, ExtraAttrs, FromIdent};
use crate::models::{FieldMapper, FieldMeta};

#[derive(Debug, FromMeta)]
pub struct IgnoreField {
    #[darling(rename = "for")]
    pub name: syn::Ident,
}

impl FromIdent for IgnoreField {
    fn form_ident(ident: syn::Ident) -> Self { Self { name: ident } }
}

#[derive(Debug, FromMeta)]
pub struct HaveFiled {
    #[darling(default)]
    pub vis: Option<Visibility>,
    #[darling(rename = "for")]
    pub name: syn::Ident,
    #[darling(rename = "rename")]
    pub to: Option<syn::Ident>,
    #[darling(default)]
    pub extra: Option<ExtraAttrs>,
}

impl FromIdent for HaveFiled {
    fn form_ident(ident: syn::Ident) -> Self {
        Self {
            name: ident,
            to: None,
            extra: None,
            vis: None,
        }
    }
}

#[derive(Debug)]
pub enum FieldInfo {
    Want(HaveFiled),
    Ignore(IgnoreField),
    Having(HaveFiled),
}

impl FromMeta for FieldInfo {
    fn from_nested_meta(item: &NestedMeta) -> darling::Result<Self> {
        match item {
            NestedMeta::Meta(Meta::List(meta_list)) => {
                let nest_meta_list =
                    meta_list.nested.iter().cloned().collect::<Vec<_>>();

                if meta_list.path.is_ident("want") {
                    Ok(FieldInfo::Want(load_from_meta_list(&nest_meta_list)?))
                }
                else if meta_list.path.is_ident("ignore") {
                    Ok(FieldInfo::Ignore(load_from_meta_list(
                        &nest_meta_list,
                    )?))
                }
                else if meta_list.path.is_ident("having") {
                    Ok(FieldInfo::Having(load_from_meta_list(
                        &nest_meta_list,
                    )?))
                }
                else {
                    let name =
                        meta_list.path.get_ident().unwrap().to_string();
                    Err(darling::Error::unknown_field(&name))
                }
            }
            NestedMeta::Lit(_) => {
                Err(darling::Error::unsupported_format("lit"))
            }
            NestedMeta::Meta(_) => {
                Err(darling::Error::unsupported_format("meta"))
            }
        }
    }
}

impl FieldInfo {
    pub fn get_ident(&self) -> &syn::Ident {
        match self {
            FieldInfo::Having(HaveFiled { name, .. })
            | FieldInfo::Want(HaveFiled { name, .. })
            | FieldInfo::Ignore(IgnoreField { name, .. }) => name,
        }
    }

    pub fn to_field_meta(self, name: Ident) -> FieldMeta {
        match self {
            FieldInfo::Want(HaveFiled { to, extra, vis, .. })
            | FieldInfo::Having(HaveFiled { to, extra, vis, .. }) => {
                let name = FieldMapper::from_names(name, to);
                let vis = Some(vis.unwrap_or(pub_vis()));
                let extra =
                    extra.map(|extras| extras.inner).unwrap_or_default();

                FieldMeta { vis, name, extra }
            }
            FieldInfo::Ignore(IgnoreField { .. }) => {
                let name = FieldMapper::Raw(name);
                let vis = None;
                let extra = vec![];

                FieldMeta { vis, name, extra }
            }
        }
    }
}

#[derive(Default)]
pub struct WantFieldInfo {
    pub inner: Vec<FieldInfo>,
}

impl std::fmt::Debug for WantFieldInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WantFieldInfo")
            .field("inner", &self.inner)
            .finish()
    }
}

impl FromMeta for WantFieldInfo {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let mut inner = Vec::with_capacity(items.len());
        for item in items {
            let v = <FieldInfo as FromMeta>::from_nested_meta(item)?;
            inner.push(v)
        }
        Ok(Self { inner })
    }
}
