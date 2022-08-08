use darling::{ast, util::Ignored, FromDeriveInput};
use syn::Ident;

use super::variant_info::VariantInfo;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(status_err), supports(enum_any))]
pub struct StatusErrorDeriveInfo {
    pub(crate) ident: Ident,
    pub(crate) data: ast::Data<VariantInfo, Ignored>,
    #[darling(default)]
    pub(crate) resp_err: bool,
}

#[cfg(test)]
mod test {
    use darling::FromDeriveInput;
    use syn::DeriveInput;

    use super::StatusErrorDeriveInfo;
    #[test]
    fn test_derive_input() {
        let derive_input: DeriveInput = syn::parse_str(
            r#"
            #[derive(StatusErr)]
            pub enum Error{
                UrlParse(url::ParseError),
                #[status_err(
                    err(
                        resp_msg = "token [{0:?}] 未找到",
                        err_code = 0x0023,
                        prefix = "Prefix::CHECK",
                        http_code = "StatusCode::NOT_FOUND",
                    )
                )]
                TokenNotFound(&'static str)
            }
            "#,
        )
        .unwrap();

        let s =
            <StatusErrorDeriveInfo as FromDeriveInput>::from_derive_input(
                &derive_input,
            )
            .unwrap();

        println!("{s:?}")
    }
}
