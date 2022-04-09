use darling::FromMeta;
use syn::{Attribute, Meta, NestedMeta, Token, VisPublic, Visibility};

use super::FromIdent;

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

pub(super) fn load_from_meta_list<T: FromIdent>(
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

pub(super) fn load_meta_list_from_attrs(
    attr_name: &str, attrs: &impl AsRef<[Attribute]>,
) -> Vec<NestedMeta> {
    attrs
        .as_ref()
        .into_iter()
        .filter_map(|attr| attr.parse_meta().ok())
        .filter(|meta| meta.path().is_ident(attr_name))
        .filter_map(|meta| {
            match meta {
                Meta::List(meta_inner) => Some(meta_inner.nested),
                _ => None,
            }
        })
        .map(|meta| meta.into_iter().collect::<Vec<_>>())
        .reduce(|mut l, r| {
            l.extend(r);
            l
        })
        .unwrap_or_default()
}

pub fn pub_vis() -> Visibility {
    Visibility::Public(VisPublic {
        pub_token: <Token![pub]>::default(),
    })
}
