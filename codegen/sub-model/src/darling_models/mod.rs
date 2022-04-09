mod derive_input;
mod item_attr_input;
mod utils;
mod want;

use darling::FromMeta;
pub use derive_input::{DeriveAttrInfo, DeriveField};
pub use utils::pub_vis;
use utils::{load_meta_list_from_attrs, ExtraAttrs};
pub use want::{FieldInfo, HaveFiled, IgnoreField, WantFieldInfo};

trait FromIdent: FromMeta {
    fn form_ident(ident: syn::Ident) -> Self;
}

const ATTR_NAME: &str = "sub_model";
