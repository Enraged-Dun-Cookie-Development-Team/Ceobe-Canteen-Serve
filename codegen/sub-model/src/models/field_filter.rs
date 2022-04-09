use proc_macro2::Ident;
use syn::{NestedMeta, Visibility};

#[derive(Debug, Clone)]
pub enum FieldMapper {
    Raw(Ident),
    Mapping { from: Ident, to: Ident },
}

impl FieldMapper {
    pub fn from_names(from: Ident, to: Option<Ident>) -> Self {
        match to {
            Some(to) => FieldMapper::Mapping { from, to },
            None => FieldMapper::Raw(from),
        }
    }

    fn get_sub_ident(&self) -> &Ident {
        match self {
            FieldMapper::Raw(ident, ..) => ident,
            FieldMapper::Mapping { to: ident, .. } => ident,
        }
    }

    fn get_parent_ident(&self) -> &Ident {
        match self {
            FieldMapper::Raw(ident, ..)
            | FieldMapper::Mapping { from: ident, .. } => ident,
        }
    }
}
#[derive(Debug)]
pub struct FieldMeta {
    pub vis: Option<Visibility>,
    pub name: FieldMapper,
    pub extra: Vec<NestedMeta>,
}

impl FieldMeta {
    pub fn dst_ident(&self) -> Ident { self.name.get_sub_ident().clone() }

    pub fn src_ident(&self) -> Ident { self.name.get_parent_ident().clone() }
}
