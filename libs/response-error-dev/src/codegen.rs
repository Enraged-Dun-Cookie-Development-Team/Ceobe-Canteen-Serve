use std::collections::HashMap;

use http::StatusCode;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

use crate::payloads::{ErrorCfg, ErrorType};

pub struct ErrorGen<'s> {
    mark: char,
    mark_description: &'s str,
    status_code: StatusCode,
    ident: &'s str,
    description: &'s str,
    code: u16,
}

impl<'s> ToTokens for ErrorGen<'s> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ErrorGen {
            mark,
            status_code,
            ident,
            description,
            code,
            mark_description,
        } = self;
        let doc_description = format!(
            "## 响应异常  \n- `{mark}`: {mark_description}  \n- ErrorCode: \
             {mark}{code:04X}: {description}  \n- HttpCode: {status_code}"
        );
        let status_code = status_code.as_u16();
        let ident = format_ident!("{ident}Error");

        let code = quote! {
            #[doc=#doc_description]
            pub struct #ident;

            impl super::GenError for #ident{
                fn mark(&self)->char{ #mark}

                fn status_code(&self)->http::StatusCode {http::StatusCode::from_u16(#status_code).unwrap()}

                fn description(&self)->&'static str{#description}

                fn code(&self)->u16 {#code}
            }
        };
        tokens.extend(code)
    }
}

impl<'s> ErrorGen<'s> {
    pub fn generate_kind_code(ident: &str, errors: &[Self]) -> TokenStream {
        let errors = errors.iter();
        let ident = format_ident!("{ident}_kind");
        let code = quote! {
            pub mod #ident{
                #(#errors)*
            }
        };
        code
    }
}

impl<'s> ErrorGen<'s> {
    pub fn from_error_type(error: &'s ErrorType) -> Vec<Self> {
        let mut out = Vec::new();

        for (code, err) in error.error.iter().enumerate() {
            let g = ErrorGen {
                mark: error.mark,
                mark_description: &error.description,
                status_code: err
                    .http_code
                    .unwrap_or(error.default_status_code),
                ident: &err.ident,
                description: &err.description,
                code: (code as u16) + 1,
            };
            out.push(g)
        }

        out
    }

    pub fn from_error_cfg(
        errors: &'s ErrorCfg,
    ) -> HashMap<&'s str, Vec<Self>> {
        let mut out = HashMap::new();

        for error in &errors.kind {
            let v = Self::from_error_type(error);
            out.insert(error.ident.as_str(), v);
        }
        out
    }
}
#[cfg(test)]
mod test {
    use crate::codegen::ErrorGen;

    #[test]
    fn test() {
        let v = include_str!("../../.././example_error_config.toml");
        let payload = toml::from_str(v).expect("Error");

        let err = ErrorGen::from_error_cfg(&payload);
        for (k, v) in err {
            let g = ErrorGen::generate_kind_code(k, &v).to_string();

            println!("{g}")
        }
    }
}
