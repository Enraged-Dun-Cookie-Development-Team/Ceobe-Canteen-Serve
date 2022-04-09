use std::collections::HashMap;

use darling::FromMeta;
use syn::{Ident, NestedMeta};

use super::{pub_vis, utils::load_from_meta_list, ExtraAttrs, FromIdent};
use crate::models::{DefaultEmpty, SubModel, WantAll};

#[derive(Debug)]
pub enum GroupIn {
    All,
    None,
}

#[derive(Debug, darling::FromMeta)]
pub struct SubModelMetaInfo {
    #[darling(default)]
    pub vis: Option<syn::Visibility>,
    pub name: Ident,
    #[darling(default)]
    pub extra: Option<ExtraAttrs>,
}

impl FromIdent for SubModelMetaInfo {
    fn form_ident(ident: syn::Ident) -> Self {
        SubModelMetaInfo {
            name: ident,
            vis: None,
            extra: None,
        }
    }
}

#[derive(Debug)]
pub struct SubModelMeta {
    pub group_in: GroupIn,
    pub inner: SubModelMetaInfo,
}

impl Into<SubModel> for SubModelMeta {
    fn into(self) -> SubModel {
        let name = self.inner.name;
        let extra = self.inner.extra.unwrap_or_default().inner;
        let vis = pub_vis();
        match self.group_in {
            GroupIn::All => {
                SubModel::DefaultAll(WantAll {
                    name,
                    ignores: vec![],
                    having_change: vec![],
                    extra,
                    vis,
                })
            }
            GroupIn::None => {
                SubModel::DefaultEmpty(DefaultEmpty {
                    name,
                    wants: vec![],
                    extra,
                    vis,
                })
            }
        }
    }
}

impl FromMeta for SubModelMeta {
    fn from_nested_meta(item: &NestedMeta) -> darling::Result<Self> {
        match item {
            NestedMeta::Meta(meta) => {
                match meta {
                    syn::Meta::List(meta_list) => {
                        let nest_meta_list = meta_list
                            .nested
                            .iter()
                            .cloned()
                            .collect::<Vec<_>>();
                        if meta_list.path.is_ident("all") {
                            Ok(SubModelMeta {
                                group_in: GroupIn::All,
                                inner: load_from_meta_list(&nest_meta_list)?,
                            })
                        }
                        else if meta_list.path.is_ident("none") {
                            Ok(SubModelMeta {
                                group_in: GroupIn::None,
                                inner: load_from_meta_list(&nest_meta_list)?,
                            })
                        }
                        else {
                            let name = meta_list
                                .path
                                .get_ident()
                                .unwrap()
                                .to_string();
                            Err(darling::Error::unknown_field(&name))
                        }
                    }
                    syn::Meta::Path(_) => {
                        Err(darling::Error::unsupported_format("path"))
                    }
                    syn::Meta::NameValue(_) => {
                        Err(darling::Error::unsupported_format("name_value"))
                    }
                }
            }
            NestedMeta::Lit(_) => {
                Err(darling::Error::unsupported_format("lit"))
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct SubModelMetaList {
    pub inner: Vec<SubModelMeta>,
}

impl SubModelMetaList {
    pub fn to_models_map(self) -> HashMap<Ident, SubModel> {
        self.inner
            .into_iter()
            .map(|model| (model.inner.name.clone(), model.into()))
            .collect()
    }
}

impl FromMeta for SubModelMetaList {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let mut inner = Vec::with_capacity(items.len());
        for item in items {
            let v = FromMeta::from_nested_meta(item)?;
            inner.push(v)
        }
        Ok(Self { inner })
    }
}
