use darling::FromMeta;
use syn::{Expr, Ident};

#[derive(Debug)]
pub struct FormatStr {
    pub(crate) raw_fmt: String,
    pub(crate) args: Vec<Args>,
}

impl FromMeta for FormatStr {
    fn from_nested_meta(item: &syn::NestedMeta) -> darling::Result<Self> {
        (match *item {
            syn::NestedMeta::Lit(ref lit) => {
                Ok(Self {
                    raw_fmt: String::from_value(lit)?,
                    args: Default::default(),
                })
            }
            syn::NestedMeta::Meta(ref mi) => Self::from_meta(mi),
        })
        .map_err(|e| e.with_span(item))
    }

    fn from_list(items: &[syn::NestedMeta]) -> darling::Result<Self> {
        let mut iter = items.iter();
        let fmt_str = iter
            .next()
            .ok_or_else(|| darling::Error::missing_field("Format Str"))?;
        let fmt_str = String::from_nested_meta(fmt_str)?;

        let v = iter.map(Args::from_nested_meta).fold(
            darling::Result::Ok(Vec::new()),
            |vec, args| {
                let args = args?;
                vec.map(|mut vec| {
                    vec.push(args);
                    vec
                })
            },
        )?;

        Ok(Self {
            raw_fmt: fmt_str,
            args: v,
        })
    }

    fn from_string(value: &str) -> darling::Result<Self> {
        Ok(Self {
            raw_fmt: value.to_owned(),
            args: vec![],
        })
    }
}
#[derive(Debug)]
pub(crate) enum Args {
    List(Expr),
    NameValue(Ident, Expr),
}

impl FromMeta for Args {
    fn from_nested_meta(item: &syn::NestedMeta) -> darling::Result<Self> {
        (match *item {
            syn::NestedMeta::Lit(ref lit) => {
                Ok(Self::List(Expr::from_value(lit)?))
            }
            syn::NestedMeta::Meta(ref mi) => Self::from_meta(mi),
        })
        .map_err(|e| e.with_span(item))
    }

    fn from_meta(item: &syn::Meta) -> darling::Result<Self> {
        (match *item {
            syn::Meta::NameValue(ref value) => Self::from_value(&value.lit),
            _ => {
                Err(darling::Error::unexpected_type(
                    "only accept NameValue or Literal",
                ))
            }
        })
        .map_err(|e| e.with_span(item))
    }

    fn from_string(value: &str) -> darling::Result<Self> {
        let expr = Expr::from_string(value)?;
        Ok(Self::List(expr))
    }
}
