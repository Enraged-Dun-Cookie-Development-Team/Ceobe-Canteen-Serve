use darling::FromMeta;
use proc_macro2::Ident;
use syn::{Meta, NestedMeta};

use super::FromIdent;

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
    #[darling(rename = "for")]
    pub name: syn::Ident,
    #[darling(default)]
    #[darling(rename = "rename")]
    pub to: Option<syn::Ident>,
    #[darling(default)]
    pub extra: Option<ExtraAttrs>,
}

impl FromIdent for WantField {
    fn form_ident(ident: syn::Ident) -> Self {
        Self {
            name: ident,
            to: None,
            extra: None,
        }
    }
}

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
        }
    }
}

fn load_from_meta_list<T: FromIdent>(
    meta_list: &impl AsRef<[NestedMeta]>,
) -> darling::Result<T> {
    // try load direct
    <T as FromMeta>::from_list(meta_list.as_ref()).or_else(|err| {
        meta_list
            .as_ref()
            .first()
            // if len of vec nest meta is 0,no try
            .ok_or(err)
            .and_then(|meta| {
                // try load ident only
                <syn::Ident as FromMeta>::from_nested_meta(meta)
                    // mapping to T
                    .map(<T as FromIdent>::form_ident)
            })
    })
}

#[derive(Debug)]
pub enum FieldInfo {
    Want(WantField),
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
            | FieldInfo::Want(WantField { name, .. })
            | FieldInfo::Ignore(IgnoreField { name, .. }) => name,
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

#[derive(Debug, Default)]
pub struct ExtraAttrs {
    pub inner: Vec<NestedMeta>,
}

impl FromMeta for ExtraAttrs {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        Ok(Self {
            inner: items.to_owned(),
        })
    }

    fn from_nested_meta(item: &NestedMeta) -> darling::Result<Self> {
        Ok(Self {
            inner: vec![item.clone()],
        })
    }

    fn from_string(value: &str) -> darling::Result<Self> {
        let meta = Meta::from_string(value)?;
        Self::from_meta(&meta)
    }
}
