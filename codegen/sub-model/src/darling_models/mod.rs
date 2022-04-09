mod derive_input;
mod want;

use darling::FromMeta;
pub use derive_input::{DeriveAttrInfo, DeriveField};
pub use want::{
    FieldInfo, IgnoreField, SubModelsName, WantField, WantFieldInfo,
    HaveFiled
};


trait FromIdent:FromMeta {
    fn form_ident(ident:syn::Ident)->Self;
}