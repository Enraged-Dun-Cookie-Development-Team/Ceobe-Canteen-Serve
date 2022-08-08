use darling::{ast, FromMeta, FromVariant};
use syn::{spanned::Spanned, Expr, Ident, LitInt};

use super::field_info::FieldInfo;

#[derive(Debug, FromMeta)]
pub struct NormalVariant {
    pub(crate) resp_msg: Option<String>,
    #[darling(rename = "err_code")]
    pub(crate) error_code: LitInt,
    pub(crate) prefix: Expr,
    pub(crate) http_code: Option<Expr>,
}

#[derive(Debug, Default)]
pub enum VariantInnerInfo {
    #[default]
    Transparent,
    Create(NormalVariant),
}

impl FromMeta for VariantInnerInfo {
    fn from_list(items: &[syn::NestedMeta]) -> darling::Result<Self> {
        Ok(Self::Create(NormalVariant::from_list(items)?))
    }

    fn from_meta(item: &syn::Meta) -> darling::Result<Self> {
        match item {
            syn::Meta::Path(ident) => {
                if ident.is_ident("transparent") {
                    Ok(Self::Transparent)
                }
                else {
                    Err(syn::Error::new(
                        ident.span(),
                        "only support `transparent`",
                    )
                    .into())
                }
            }
            syn::Meta::List(ref value) => {
                Self::from_list(
                    &value.nested.iter().cloned().collect::<Vec<_>>()[..],
                )
            }
            syn::Meta::NameValue(v) => Self::from_value(&v.lit),
        }
    }

    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        match value {
            syn::Lit::Str(str) => {
                if str.value() == "transparent" {
                    Ok(Self::Transparent)
                }
                else {
                    Err(darling::Error::unexpected_lit_type(value))
                }
            }
            _ => Err(darling::error::Error::unexpected_lit_type(value)),
        }
        .map_err(|e| e.with_span(value))
    }
}

#[derive(Debug, FromVariant)]
#[darling(attributes(status_err))]
pub struct VariantInfo {
    pub(crate) ident: Ident,
    pub(crate) fields: ast::Fields<FieldInfo>,
    #[darling(default)]
    pub(crate) err: VariantInnerInfo,
}

impl VariantInfo {
    pub fn checking(&self) -> syn::Result<()> {
        if matches!(self.err, VariantInnerInfo::Transparent)
            && self.fields.is_tuple()
            && self.fields.len() == 1
        {
        }
        else {
            Err(syn::Error::new(
                self.ident.span(),
                "Transparent Only Support Tuple with 1 Field",
            ))?
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use darling::FromVariant;
    use syn::Variant;

    use super::VariantInfo;

    #[test]
    fn test_from_meta() {
        let meta: Variant = syn::parse_str(
            r#"
        #[status_err(
            err(
                resp_msg = "avc",
                err_code = 0x0012,
                prefix = "Prefix::CHECK"
            )
        )]
        Var(String)
        "#,
        )
        .unwrap();
        let v = VariantInfo::from_variant(&meta).expect("V");

        println!("{v:?}")
    }
    #[test]
    fn test_transparent() {
        let meta: Variant = syn::parse_str(
            r#"
        #[status_err]
        Var(String)
        "#,
        )
        .unwrap();

        let v = VariantInfo::from_variant(&meta).expect("V");

        println!("{v:?}")
    }
}
