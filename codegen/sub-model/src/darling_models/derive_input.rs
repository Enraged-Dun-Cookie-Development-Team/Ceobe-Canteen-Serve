use darling::FromMeta;
use syn::Meta;

use super::want::{SubModelsName, WantFieldInfo};

#[derive(Debug, Default, darling::FromDeriveInput)]
#[darling(attributes(sub_model), default)]
pub struct DeriveAttrInfo {
    #[darling(default)]
    pub all: SubModelsName,
    #[darling(default)]
    pub none: SubModelsName,
}
#[derive(Debug)]
pub struct DeriveField {
    pub field: WantFieldInfo,
}

impl darling::FromField for DeriveField {
    fn from_field(field: &syn::Field) -> darling::Result<Self> {
        let metas = {
            let mut meta = Vec::new();
            for attr in field.attrs.clone() {
                let v = attr.parse_meta()?;
                meta.push(v)
            }
            meta
        }
        .into_iter()
        .filter(|meta| meta.path().is_ident("sub_model"))
        .filter_map(|meta| {
            match meta {
                Meta::List(meta) => Some(meta),
                _ => None,
            }
        })
        .map(|meta| meta.nested.into_iter().collect::<Vec<_>>())
        .reduce(|mut l, r| {
            l.extend(r);
            l
        })
        .unwrap_or_default();

        let inner = WantFieldInfo::from_list(&metas)?;

        Ok(Self { field: inner })
    }
}
