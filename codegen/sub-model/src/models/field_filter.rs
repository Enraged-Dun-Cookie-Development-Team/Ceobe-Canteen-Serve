use proc_macro2::Ident;

pub enum FieldMapper {
    Raw(Ident),
    Mapping { from: Ident, to: Ident },
}

impl FieldMapper {
    pub fn get_sub_ident(&self) -> &Ident {
        match self {
            FieldMapper::Raw(ident) => ident,
            FieldMapper::Mapping { to: ident, .. } => ident,
        }
    }

    pub fn get_parent_ident(&self) -> &Ident {
        match self {
            FieldMapper::Raw(ident)
            | FieldMapper::Mapping { from: ident, .. } => ident,
        }
    }
}
