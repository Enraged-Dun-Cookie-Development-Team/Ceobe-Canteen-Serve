use darling::FromMeta;

use super::{
    item_attr_input::SubModelMetaList, load_meta_list_from_attrs,
    want::WantFieldInfo, ATTR_NAME,
};

#[derive(Debug, Default)]
pub struct DeriveAttrInfo {
    pub inner: SubModelMetaList,
}

impl darling::FromDeriveInput for DeriveAttrInfo {
    fn from_derive_input(input: &syn::DeriveInput) -> darling::Result<Self> {
        let meta_list = load_meta_list_from_attrs(ATTR_NAME, &input.attrs);
        let inner = FromMeta::from_list(&meta_list)?;
        Ok(Self { inner })
    }
}

#[derive(Debug)]
pub struct DeriveField {
    pub field: WantFieldInfo,
}

impl darling::FromField for DeriveField {
    fn from_field(field: &syn::Field) -> darling::Result<Self> {
        let meta_list = load_meta_list_from_attrs(ATTR_NAME, &field.attrs);

        let inner = WantFieldInfo::from_list(&meta_list)?;

        Ok(Self { field: inner })
    }
}
