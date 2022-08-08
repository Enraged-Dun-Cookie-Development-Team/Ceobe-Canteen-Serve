use darling::{ast::Style, ToTokens};
use quote::__private::TokenStream;
use syn::Ident;

use crate::input_loading::variant_info::{
    NormalVariant, VariantInfo, VariantInnerInfo,
};

pub struct InfoImplToken<'s> {
    error_name: &'s Ident,
}

impl VariantInfo {
    pub fn get_info_impl(err_name: &Ident) -> InfoImplToken<'_> {
        InfoImplToken {
            error_name: err_name,
        }
    }
}
impl<'s> ToTokens for InfoImplToken<'s> {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let err_name = self.error_name;

        let token = quote::quote! {
            #[inline]
            fn information(&self) -> std::borrow::Cow<'_, str>{
                format!(
                    "{} => {}",
                    stringify!(#err_name),
                    self
                ).into()
            }
        };

        tokens.extend(token)
    }
}

fn style_ignore(style: &Style) -> TokenStream {
    match style {
        Style::Tuple => quote::quote! { (..) },
        Style::Struct => quote::quote!({ .. }),
        Style::Unit => quote::quote!(),
    }
}

pub struct RespMsgImplToken<'s> {
    ident: &'s Ident,
    info: &'s VariantInnerInfo,
    style: &'s Style,
}

impl VariantInfo {
    pub fn get_resp_msg_impl(&self) -> RespMsgImplToken<'_> {
        RespMsgImplToken {
            ident: &self.ident,
            info: &self.err,
            style: &self.fields.style,
        }
    }
}

impl<'s> ToTokens for RespMsgImplToken<'s> {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let ident = self.ident;
        let style = style_ignore(self.style);
        let token = match self.info {
            VariantInnerInfo::Transparent => {
                quote::quote! {
                    Self::#ident(ref inner) => {
                        ::status_err::StatusErr::respond_msg(inner)
                    }
                }
            }
            VariantInnerInfo::Create(NormalVariant {
                resp_msg: Some(msg),
                ..
            }) => {
                quote::quote! {
                    Self::#ident #style =>{
                        std::borrow::Cow::Borrowed(#msg)
                    }
                }
            }
            VariantInnerInfo::Create(_) => {
                quote::quote! {
                    Self::#ident #style =>{
                        std::borrow::Cow::Owned(<Self as ToString>::to_string(self))
                    }
                }
            }
        };
        tokens.extend(token)
    }
}
pub struct PrefixImplToken<'s> {
    ident: &'s Ident,
    info: &'s VariantInnerInfo,
    style: &'s Style,
}

impl VariantInfo {
    pub fn get_prefix_impl(& self) -> PrefixImplToken<'_> {
        PrefixImplToken {
            ident: &self.ident,
            info: &self.err,
            style: &self.fields.style,
        }
    }
}

impl<'s> ToTokens for PrefixImplToken<'s> {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let ident = self.ident;
        let style = style_ignore(self.style);
        let token = match self.info {
            VariantInnerInfo::Transparent => {
                quote::quote! {
                    Self::#ident(ref inner) => {
                        ::status_err::StatusErr::prefix(inner)
                    }
                }
            }
            VariantInnerInfo::Create(NormalVariant { prefix, .. }) => {
                quote::quote! {
                    Self::#ident #style =>{
                        #prefix
                    }
                }
            }
        };
        tokens.extend(token)
    }
}
pub struct CodeImplToken<'s> {
    ident: &'s Ident,
    info: &'s VariantInnerInfo,
    style: &'s Style,
}

impl VariantInfo {
    pub fn get_code_impl(& self) -> CodeImplToken<'_> {
        CodeImplToken {
            ident: &self.ident,
            info: &self.err,
            style: &self.fields.style,
        }
    }
}

impl<'s> ToTokens for CodeImplToken<'s> {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let ident = self.ident;
        let style = style_ignore(self.style);
        let token = match self.info {
            VariantInnerInfo::Transparent => {
                quote::quote! {
                    Self::#ident(ref inner) => {
                        ::status_err::StatusErr::code(inner)
                    }
                }
            }
            VariantInnerInfo::Create(NormalVariant {
                error_code, ..
            }) => {
                quote::quote! {
                    Self::#ident #style =>{
                        #error_code
                    }
                }
            }
        };
        tokens.extend(token)
    }
}
pub struct HttpCodeImplToken<'s> {
    ident: &'s Ident,
    info: &'s VariantInnerInfo,
    style: &'s Style,
}

impl VariantInfo {
    pub fn get_http_code_impl(&self) -> HttpCodeImplToken<'_> {
        HttpCodeImplToken {
            ident: &self.ident,
            info: &self.err,
            style: &self.fields.style,
        }
    }
}

impl<'s> ToTokens for HttpCodeImplToken<'s> {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let ident = self.ident;
        let style = style_ignore(self.style);
        let token = match self.info {
            VariantInnerInfo::Transparent => {
                quote::quote! {
                    Self::#ident(ref inner) => {
                        ::status_err::StatusErr::http_code(inner)
                    }
                }
            }
            VariantInnerInfo::Create(NormalVariant {
                http_code: Some(code),
                ..
            }) => {
                quote::quote! {
                    Self::#ident #style =>{
                        #code
                    }
                }
            }
            VariantInnerInfo::Create(_) => {
                quote::quote! {
                    Self::#ident #style =>{
                        ::status_err::StatusErr::status(self).http_code()
                    }
                }
            }
        };
        tokens.extend(token)
    }
}
