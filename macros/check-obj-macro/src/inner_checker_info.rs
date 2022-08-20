use syn::{
    spanned::Spanned, Attribute, Field, Fields, FieldsNamed, Generics, Ident,
    ItemStruct, Type, Visibility,
};

pub struct InnerCheckerInfo {
    pub(crate) attrs: Vec<Attribute>,
    pub(crate) vis: Visibility,
    pub(crate) ident: Ident,
    pub(crate) field: Vec<InnerCheckerField>,
}

impl InnerCheckerInfo {
    pub fn from_item_struct(item: ItemStruct) -> syn::Result<Self> {
        let ItemStruct {
            attrs,
            vis,
            ident,
            generics,
            fields,
            ..
        } = item;
        Self::check_generics(generics)?;

        Ok(Self {
            attrs,
            vis,
            ident,
            field: InnerCheckerField::from_fields(fields)?,
        })
    }

    fn check_generics(generics: Generics) -> syn::Result<()> {
        if !generics.params.is_empty() {
            Err(syn::Error::new(
                generics.span(),
                "Check obj Not all Generics",
            ))
        }
        else {
            Ok(())
        }
    }
}

pub struct InnerCheckerField {
    pub(crate) attrs: Vec<Attribute>,
    pub(crate) vis: Visibility,
    pub(crate) name: Ident,
    pub(crate) ty: Type,
}

impl InnerCheckerField {
    pub fn from_fields(fields: Fields) -> syn::Result<Vec<Self>> {
        if let Fields::Named(FieldsNamed { named, .. }) = fields {
            let mut resp = Vec::with_capacity(named.len());
            for Field {
                attrs,
                vis,
                ident,
                ty,
                ..
            } in named
            {
                let ident = if let Some(ident) = ident {
                    ident
                }
                else {
                    return Err(syn::Error::new(
                        ident.span(),
                        "only accept `Named Struct`",
                    ));
                };

                let this = Self {
                    attrs,
                    vis,
                    name: ident,
                    ty,
                };

                resp.push(this)
            }
            Ok(resp)
        }
        else {
            Err(syn::Error::new(fields.span(), "only accept `Named Struct`"))
        }
    }
}
