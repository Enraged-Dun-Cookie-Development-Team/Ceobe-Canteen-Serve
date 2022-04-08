use darling::FromMeta;
use proc_macro2::Ident;
use syn::{Meta, NestedMeta};

#[derive(Debug, Default)]
pub struct SubModelsName {
    pub inner: Vec<Ident>,
}

impl FromMeta for SubModelsName {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let mut inner = Vec::with_capacity(items.len());

        for item in items {
            inner.push(Ident::from_nested_meta(item)?);
        }

        Ok(Self { inner })
    }
}

#[derive(Debug, FromMeta)]
pub struct WantField {
    #[darling(rename="for")]
    pub name: syn::Ident,
    #[darling(default)]
    #[darling(rename="rename")]
    pub to: Option<syn::Ident>,
}
#[derive(Debug, FromMeta)]
pub struct IgnoreField {
    #[darling(rename="for")]
    pub name: syn::Ident,
}

#[derive(Debug)]
pub enum FieldInfo {
    Want(WantField),
    Ignore(IgnoreField),
}

impl FromMeta for FieldInfo {
    fn from_nested_meta(item: &NestedMeta) -> darling::Result<Self> {
        match item {
            NestedMeta::Meta(Meta::List(meta_list)) => {
                if meta_list.path.is_ident("want") {
                    Ok(FieldInfo::Want(<WantField as FromMeta>::from_list(
                        &meta_list.nested.iter().cloned().collect::<Vec<_>>()
                            [..],
                    )?))
                }
                else if meta_list.path.is_ident("ignore") {
                    Ok(FieldInfo::Ignore(
                        <IgnoreField as FromMeta>::from_list(
                            &meta_list
                                .nested
                                .iter()
                                .cloned()
                                .collect::<Vec<_>>()[..],
                        )?,
                    ))
                }
                else {
                    let name =
                        meta_list.path.get_ident().unwrap().to_string();
                    Err(darling::Error::unknown_field(&name))
                }
            }
            NestedMeta::Lit(_) => {
                Err(darling::Error::unsupported_format("Lit"))
            }
            NestedMeta::Meta(_) => {
                Err(darling::Error::unsupported_format("Not MetaList"))
            }
        }
    }
}

impl FieldInfo {
    pub fn get_ident(&self) -> &syn::Ident {
        match self {
            FieldInfo::Want(WantField { name, .. })
            | FieldInfo::Ignore(IgnoreField { name, .. }) => name,
        }
    }
}

#[derive(Debug, Default)]
pub struct WantFieldInfo {
    pub inner: Vec<FieldInfo>,
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


pub struct ExtraAttrs{
    inner:Vec<NestedMeta>
}

