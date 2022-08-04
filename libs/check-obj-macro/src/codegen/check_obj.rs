use quote::{format_ident, ToTokens};
use syn::{Attribute, Ident, Type, Visibility};

use super::inner_checker::InnerChecker;
use crate::{
    checker_info::CheckerInfo, inner_checker_info::InnerCheckerInfo,
};

pub struct CheckObj {
    uncheck: Ident,
    checked: Type,
    checker: Ident,
    error: Type,

    checker_vis: Visibility,
    attrs: Vec<Attribute>,
    inner_checkers: Vec<InnerChecker>,
}

impl From<(CheckerInfo, InnerCheckerInfo)> for CheckObj {
    fn from(
        (
            CheckerInfo {
                uncheck_name,
                checked,
                error,
            },
            InnerCheckerInfo {
                attrs,
                vis,
                ident,
                field,
            },
        ): (CheckerInfo, InnerCheckerInfo),
    ) -> Self {
        Self {
            uncheck: uncheck_name,
            checked,
            checker: ident,
            error,
            checker_vis: vis,
            attrs,
            inner_checkers: field.into_iter().map(Into::into).collect(),
        }
    }
}

impl ToTokens for CheckObj {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let Self {
            uncheck,
            checked,
            checker,
            error,
            checker_vis,
            attrs,
            inner_checkers,
        } = self;
        let root = &format_ident!("this");
        let builder = &format_ident!("builder");
        let fut_token = &format_ident!("{}Fut", checker);
        let uncheck_name = &format_ident!("uncheck");
        let context = &format_ident!("cx");

        // uncheck
        let uncheck_attrs = attrs.iter();
        let uncheck_iter =
            inner_checkers.iter().map(|v| v.get_uncheck_field());
        let uncheck_token = quote::quote! {
            #(#[#uncheck_attrs])*
            #checker_vis struct #uncheck {
                #(#uncheck_iter),*
            }
        };
        tokens.extend(uncheck_token);

        // checker fut
        let fut_field_final = inner_checkers
            .iter()
            .map(|v| v.get_checking_fut_final(root, builder));
        let fut_field =
            inner_checkers.iter().map(|v| v.get_checking_fut_field());
        let fut_poll = inner_checkers
            .iter()
            .map(|v| v.get_checking_poll(root, context, error));

        let checker_fut_token = quote::quote! {
            #checker_vis struct #fut_token{
                __pinned : std::marker::PhantomPinned,
                #(
                    #fut_field
                ),*
            }

            impl std::future::Future for #fut_token {
                type Output = Result<#checked, #error>;

                fn poll(self:std::pin::Pin<&mut Self>, #context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output>{
                    let #root = unsafe{ self.get_unchecked_mut() };
                    // poll all
                    #(
                        #fut_poll
                    );*

                    // all finish
                    let #builder = <#checked>::builder();

                    #(
                        #fut_field_final
                    )*

                    std::task::Poll::Ready(Ok(#builder.build()))
                }
            }
        };

        tokens.extend(checker_fut_token);

        // checker
        let check_err_bounds =
            inner_checkers.iter().map(|v| v.get_check_err_bound(error));
        let check_bounds = inner_checkers.iter().map(|v| v.get_check_bound());
        let fut_create = inner_checkers
            .iter()
            .map(|v| v.get_checking_fut_create(uncheck_name, v.get_name()));
        let checker_args =
            inner_checkers.iter().map(|v| v.get_checking_args());
        let arg_checker_args = inner_checkers.iter().map(|v| v.get_name());
        let checker_token = quote::quote! {
            #checker_vis struct #checker;

            impl checker::Checker for #checker
            where
                #(
                    #check_bounds,
                    #check_err_bounds
                ),*
            {
                type Unchecked = #uncheck;

                type Args = ( #(#checker_args),*, );

                type Checked = #checked;

                type Err = #error;

                type Fut = #fut_token;

                fn check(( #(#arg_checker_args),*, ):Self::Args, #uncheck_name: Self::Unchecked)->Self::Fut{
                    #fut_token{
                        __pinned : std::marker::PhantomPinned,
                        #(
                            #fut_create
                        ),*
                    }
                }
            }
        };

        tokens.extend(checker_token);
    }
}
