use darling::ToTokens;
use syn::Ident;

use crate::input_loading::{
    derive_info::StatusErrorDeriveInfo, variant_info::VariantInfo,
};

pub struct StatusErrImpl {
    ident: Ident,
    vars: Vec<VariantInfo>,
    resp_err: bool,
}

impl StatusErrorDeriveInfo {
    pub fn checking(self) -> syn::Result<StatusErrImpl> {
        if !self.data.is_enum() {
            Err(syn::Error::new(self.ident.span(), "Only Support Enum"))?
        }
        let vars = self.data.take_enum().unwrap();

        for v in vars.iter() {
            v.checking()?;
        }

        Ok(StatusErrImpl {
            ident: self.ident,
            vars,
            resp_err: self.resp_err,
        })
    }
}

impl ToTokens for StatusErrImpl {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let name = &self.ident;

        // fn information
        let info_impl = VariantInfo::get_info_impl(&self.ident);
        let info_impl = quote::quote! {#info_impl};

        // fn resp_msg
        let resp_msg_impl_iter =
            self.vars.iter().map(VariantInfo::get_resp_msg_impl);
        let resp_msg_impl = quote::quote! {
            #[inline]
            fn respond_msg(&self) -> std::borrow::Cow<'_, str> {
                match self{
                    #(#resp_msg_impl_iter),*
                }
            }
        };

        let prefix_impl_iter =
            self.vars.iter().map(VariantInfo::get_prefix_impl);
        let prefix_impl = quote::quote! {
            #[inline]
            fn prefix(&self) -> ::status_err::ErrPrefix {
                match self{
                    #(#prefix_impl_iter),*
                }
            }
        };

        let code_impl_iter = self.vars.iter().map(VariantInfo::get_code_impl);
        let code_impl = quote::quote! {
            #[inline]
            fn code(&self) -> u16 {
                match self{
                    #(#code_impl_iter),*
                }
            }
        };

        let http_code_impl_iter =
            self.vars.iter().map(VariantInfo::get_http_code_impl);
        let http_code_impl = quote::quote! {
            #[inline]
            fn http_code(&self) -> ::status_err::HttpCode {
                match self {
                    #(#http_code_impl_iter),*
                }
            }
        };

        let impl_token = quote::quote! {
            impl status_err::StatusErr for #name {
                #info_impl

                #resp_msg_impl

                #prefix_impl

                #code_impl

                #http_code_impl
            }
        };

        tokens.extend(impl_token);

        if self.resp_err {
            let token = quote::quote! {
                ::status_err::resp_error_impl!(#name);
            };
            tokens.extend(token)
        }
    }
}
