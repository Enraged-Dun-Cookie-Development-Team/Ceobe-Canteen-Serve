use quote::{quote, ToTokens};
use syn::{Attribute, Ident, Type, Visibility};

use crate::inner_checker_info::InnerCheckerField;

pub struct InnerChecker {
    attrs: Vec<Attribute>,
    vis: Visibility,
    checker: Type,
    field_name: Ident,
}

impl InnerChecker {
    pub fn get_name(&self) -> &Ident { &self.field_name }

    pub fn get_uncheck_field(&self) -> UncheckField<'_> {
        UncheckField { inner: self }
    }

    pub fn get_checking_args(&self) -> CheckingArgs<'_> {
        CheckingArgs {
            checker: &self.checker,
        }
    }

    pub fn get_check_bound(&self) -> CheckerBound<'_> {
        CheckerBound {
            checker: &self.checker,
        }
    }

    pub fn get_check_err_bound<'c, 'e>(
        &'c self, err_ty: &'e Type,
    ) -> CheckerErrBound<'c, 'e> {
        CheckerErrBound {
            checker: &self.checker,
            error: err_ty,
        }
    }

    pub fn get_checking_fut_field(&self) -> CheckingFutField<'_> {
        CheckingFutField {
            checker: &self.checker,
            name: &self.field_name,
        }
    }

    pub fn get_checking_fut_create<'c, 'r>(
        &'c self, uncheck: &'r Ident, args: &'r Ident,
    ) -> CheckingFutCreate<'c, 'r> {
        CheckingFutCreate {
            name: &self.field_name,
            uncheck,
            arg: args,
        }
    }

    pub fn get_checking_poll<'c, 'r>(
        &'c self, root: &'r Ident, cx: &'r Ident, error: &'r Type,
    ) -> CheckingPoll<'c, 'r> {
        CheckingPoll {
            name: &self.field_name,
            root,
            cx,
            error,
        }
    }

    pub fn get_checking_fut_final<'c, 'r>(
        &'c self, root: &'r Ident, builder: &'r Ident,
    ) -> CheckingFutFinal<'c, 'r> {
        CheckingFutFinal {
            name: &self.field_name,
            root,
            builder,
        }
    }

    pub fn get_checking_fut_sync_bound(&'c self) -> SyncFutureBound<'_> {
        SyncFutureBound {
            check: &self.checker,
        }
    }

    pub fn get_sync_fut_unwrap<'c, 'r>(
        &'c self, root: &'r Ident, builder: &'r Ident,
    ) -> SyncCheckFuture<'c, 'r> {
        SyncCheckFuture {
            name: &self.field_name,
            root,
            builder,
        }
    }
}

impl From<InnerCheckerField> for InnerChecker {
    fn from(
        InnerCheckerField {
            attrs,
            vis,
            name,
            ty,
        }: InnerCheckerField,
    ) -> Self {
        Self {
            attrs,
            vis,
            checker: ty,
            field_name: name,
        }
    }
}
/// 生成check obj 的uncheck 中的每一个field
pub struct UncheckField<'c> {
    inner: &'c InnerChecker,
}

impl<'c> ToTokens for UncheckField<'c> {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let attr = self.inner.attrs.iter();
        let vis = &self.inner.vis;
        let name = &self.inner.field_name;
        let checker = &self.inner.checker;
        let token = quote! {
            #(#attr)*
            #vis #name : checker::CheckRequire< #checker >
        };

        tokens.extend(token);
    }
}

/// 生成checker 的 args
pub struct CheckingArgs<'c> {
    checker: &'c Type,
}

impl<'c> ToTokens for CheckingArgs<'c> {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let checker = self.checker;
        let token = quote! {
            < #checker  as checker::Checker >::Args
        };

        tokens.extend(token)
    }
}

/// 生成内部checker 的依赖
pub struct CheckerBound<'c> {
    checker: &'c Type,
}

impl<'c> ToTokens for CheckerBound<'c> {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let checker = self.checker;
        let token = quote! {
            #checker : checker::Checker
        };

        tokens.extend(token)
    }
}
/// 生成 内部checker 异常能够转换为指定类型的异常的约束
pub struct CheckerErrBound<'c, 'e> {
    checker: &'c Type,
    error: &'e Type,
}

impl<'c, 'e> ToTokens for CheckerErrBound<'c, 'e> {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let checker = self.checker;
        let err = self.error;
        let token = quote! {
            < #checker as checker::Checker >::Err : std::convert::Into< #err >
        };
        tokens.extend(token)
    }
}

/// 生成 checker Fut 的每一个field
pub struct CheckingFutField<'c> {
    checker: &'c Type,
    name: &'c Ident,
}

impl<'c> ToTokens for CheckingFutField<'c> {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let checker = self.checker;
        let name = self.name;
        let token = quote! {
            #name : checker::CheckFut< #checker >
        };
        tokens.extend(token);
    }
}

pub struct CheckingFutCreate<'c, 'r> {
    name: &'c Ident,
    uncheck: &'r Ident,
    arg: &'r Ident,
}

impl<'c, 'r> ToTokens for CheckingFutCreate<'c, 'r> {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let Self { name, uncheck, arg } = self;
        let token = quote! {
            #name : #uncheck.#name.into_check_fut(#arg)
        };
        tokens.extend(token)
    }
}

/// 生成在future 里面poll 执行checker代码
pub struct CheckingPoll<'c, 'r> {
    name: &'c Ident,
    root: &'r Ident,
    cx: &'r Ident,
    error: &'r Type,
}

impl<'c, 'r> ToTokens for CheckingPoll<'c, 'r> {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let Self {
            name,
            root,
            cx,
            error,
        } = self;
        let token = quote! {
            // get the field
            let __check_field = &mut #root.#name;
            // not finish poll it
            if !__check_field.is_finish(){
                // pin it
                let __check_field = unsafe {std::pin::Pin::new_unchecked(__check_field)};
                //poll
                match std::future::Future::poll(__check_field, #cx){
                    // the task done
                    std::task::Poll::Ready(Ok(()))=>(),
                    // error occur
                    std::task::Poll::Ready(Err(err))=>{
                        return std::task::Poll::Ready(Err(<#error>::from(err)));
                    }
                    // pending
                    std::task::Poll::Pending=>{
                        return std::task::Poll::Pending;
                    }
                }
            }
        };
        tokens.extend(token)
    }
}

/// 全部checker 任务完成，生成最终的checked
pub struct CheckingFutFinal<'c, 'r> {
    name: &'c Ident,
    root: &'r Ident,
    builder: &'r Ident,
}

impl<'c, 'r> ToTokens for CheckingFutFinal<'c, 'r> {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let Self {
            name,
            root,
            builder,
        } = self;
        let token = quote! {
            let #builder = #builder.#name(#root.#name.take());
        };
        tokens.extend(token)
    }
}

pub struct SyncCheckFuture<'c, 'r> {
    name: &'c Ident,
    root: &'r Ident,
    builder: &'r Ident,
}

impl<'c, 'r> ToTokens for SyncCheckFuture<'c, 'r> {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let Self {
            name,
            root,
            builder,
        } = self;
        let token = quote! {
            let #builder = #builder.#name(#root.#name.into_inner()?);
        };
        tokens.extend(token);
    }
}

pub struct SyncFutureBound<'c> {
    check: &'c Type,
}

impl<'c> ToTokens for SyncFutureBound<'c> {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let ty = self.check;

        let token = quote! {
            < #ty  as checker::Checker >::Fut : checker::SyncFuture
        };
        tokens.extend(token)
    }
}
