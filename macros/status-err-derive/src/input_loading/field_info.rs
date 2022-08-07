use darling::FromField;
use syn::{Ident, Type};

#[derive(Debug, FromField)]
pub struct FieldInfo {
    pub(crate) ident: Option<Ident>,
    pub(crate) ty: Type,
}
