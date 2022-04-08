use super::want::{SubModelsName, WantFieldInfo};

#[derive(Debug,Default, darling::FromDeriveInput)]
#[darling(attributes(sub_model), default)]
pub struct DeriveAttrInfo {
    #[darling(default)]
    pub all: SubModelsName,
    #[darling(default)]
    pub none: SubModelsName,
}
#[derive(Debug, darling::FromField)]
#[darling(attributes(sub_model))]
pub struct DeriveField {
    #[darling(default)]
    pub field: WantFieldInfo,
}
