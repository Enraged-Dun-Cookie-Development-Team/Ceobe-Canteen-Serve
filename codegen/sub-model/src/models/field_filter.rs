use proc_macro2::Ident;
use syn::NestedMeta;

#[derive(Debug, Clone)]
pub enum FieldMapper {
    Raw(Ident, Vec<NestedMeta>),
    Mapping {
        from: Ident,
        to: Ident,
        extra: Vec<NestedMeta>,
    },
}

impl FieldMapper {
    pub fn get_sub_ident(&self) -> &Ident {
        match self {
            FieldMapper::Raw(ident, _) => ident,
            FieldMapper::Mapping { to: ident, .. } => ident,
        }
    }

    pub fn get_parent_ident(&self) -> &Ident {
        match self {
            FieldMapper::Raw(ident, _)
            | FieldMapper::Mapping { from: ident, .. } => ident,
        }
    }

    pub fn get_extra(&self) -> Vec<NestedMeta> {
        match self {
            FieldMapper::Raw(_, extra)
            | FieldMapper::Mapping { extra, .. } => extra.to_owned(),
        }
    }
}
