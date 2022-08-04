use syn::{
    parse::{Lookahead1, Parse, ParseStream},
    Ident, Token, TypePath,
};
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub(crate) struct CheckerInfo {
    pub(crate) uncheck_name: syn::Ident,
    pub(crate) checked: TypePath,
    pub(crate) error: TypePath,
}

impl Parse for CheckerInfo {
    // #[check_obj(uncheck=Uncheck, checked=Checked, error=Error, )]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        let (uncheck, lookahead) = parse_uncheck(&input, lookahead)?;

        let lookahead = parse_sep(&input, lookahead)?;

        let (checked, lookahead) = parse_ty(&input, lookahead, "checked")?;

        let lookahead = parse_sep(&input, lookahead)?;

        let (error, _lookahead) = parse_ty(&input, lookahead, "error")?;

        let _ = input.parse::<Token!(,)>().ok();

        Ok(Self {
            uncheck_name: uncheck,
            checked,
            error,
        })
    }
}

fn parse_name_and_eq<'a>(
    input: &'a ParseStream, lookahead: Lookahead1<'a>, name: &str,
) -> syn::Result<Lookahead1<'a>> {
    // check name eq
    if !lookahead.peek(Ident) {
        return Err(lookahead.error());
    }
    let ident: Ident = input.parse()?;
    if ident != name {
        Err(syn::Error::new(ident.span(), format!("expect `{}`", name)))?;
    }

    // check eq sign
    let _ = input.parse::<Token!(=)>()?;

    Ok(lookahead)
}
fn parse_sep<'a>(
    input: &'a ParseStream, lookahead: Lookahead1<'a>,
) -> syn::Result<Lookahead1<'a>> {
    // check `,`
    let _t = input.parse::<Token!(,)>()?;

    Ok(lookahead)
}

fn parse_uncheck<'a>(
    input: &'a ParseStream, lookahead: Lookahead1<'a>,
) -> syn::Result<(Ident, Lookahead1<'a>)> {
    let lookahead = parse_name_and_eq(input, lookahead, "uncheck")?;
    // check uncheck type name
    if !lookahead.peek(Ident) {
        return Err(lookahead.error());
    }

    let ident = input.parse::<Ident>()?;

    Ok((ident, lookahead))
}

fn parse_ty<'a>(
    input: &'a ParseStream, lookahead: Lookahead1<'a>, name: &str,
) -> syn::Result<(TypePath, Lookahead1<'a>)> {
    let lookahead = parse_name_and_eq(input, lookahead, name)?;
    // check uncheck type name

    let ty = input.parse()?;

    Ok((ty, lookahead))
}

#[cfg(test)]
mod test {
    use super::CheckerInfo;

    #[test]
    fn test_parse() {
        let token: CheckerInfo =
            syn::parse_str("uncheck=Uncheck, checked=Checked, error=Error,")
                .expect("Token Error");

        println!("{:?}", token)
    }
}
