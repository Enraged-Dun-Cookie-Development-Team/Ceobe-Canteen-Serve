use core::marker::Send;

use async_trait::async_trait;
use axum::{
    extract::{FromRequest, FromRequestParts, Path, Query, Request},
    http::request::Parts,
    Form, Json,
};
use axum_resp_result::{
    FromRequestFamily, Nil, RespError, RespResult, ToInner,
};

use crate::{Checker as DataChecker, LiteArgs};

pub struct CheckExtract<Previous, C, E>(
    pub <C::Checker as DataChecker>::Checked,
)
where
    C: CheckFetchFamily<Previous, E>;

impl<S,Previous, C, E> FromRequest<S> for CheckExtract<Previous, C, E>
where
    S: Sync + Send,
    Previous: FromRequestFamily<S>,
    Previous::Payload: FromRequest<S> + ToInner<Inner=<C::Checker as DataChecker>::Unchecked>,
    // 检查器
    C: CheckFetchFamily<Previous,E> +Sized,
    <C::Checker as DataChecker>::Unchecked: Send,
    <C::Checker as DataChecker>::Checked: Send + Sized,
    <C::Checker as DataChecker>::Args: LiteArgs + Send,
    <C::Checker as DataChecker>::Fut: Send,
    // 异常映射
    E: From<<<Previous as FromRequestFamily<S>>::Payload as FromRequest<S>>::Rejection>,
    E: From<<C::Checker as DataChecker>::Err>,
    E: RespError,
{
    type Rejection = RespResult<Nil, E>;

    fn from_request<'life0, 'async_trait>(
        req: Request, state: &'life0 S,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<Self, Self::Rejection>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async {
            async {
                let previous =
                    <Previous::Payload as FromRequest<S>>::from_request(req, state).await?.to_inner();
                let checked =
                    C::Checker::check(LiteArgs::get_arg(), previous).await?;
                Ok(Self(checked))
            }
            .await
            .map_err(RespResult::Err)
        })
    }
}

#[async_trait]
impl<S, Previous, C, E> FromRequestParts<S> for CheckExtract<Previous, C, E>
where
    S: Sync + Send,
    Previous: FromRequestFamily<S>,
    Previous::Payload: FromRequestParts<S>
        + ToInner<Inner = <C::Checker as DataChecker>::Unchecked>,
    // 检查器
    C: CheckFetchFamily<Previous, E> + Sized,
    <C::Checker as DataChecker>::Unchecked: Send,
    <C::Checker as DataChecker>::Checked: Send + Sized,
    <C::Checker as DataChecker>::Args: LiteArgs + Send,
    <C::Checker as DataChecker>::Fut: Send,
    // 异常映射
    E: From<
        <<Previous as FromRequestFamily<S>>::Payload as FromRequestParts<
            S,
        >>::Rejection,
    >,
    E: From<<C::Checker as DataChecker>::Err>,
    E: RespError,
{
    type Rejection = RespResult<Nil, E>;

    async fn from_request_parts(
        parts: &mut Parts, state: &S,
    ) -> Result<Self, Self::Rejection> {
        async {
                    let previous =
                        <Previous::Payload as FromRequestParts<S>>::from_request_parts(parts, state).await?.to_inner();
                    let checked =
                        C::Checker::check(LiteArgs::get_arg(), previous).await?;
                    Ok(Self(checked))
                }
                    .await
                    .map_err(RespResult::Err)
    }
}
pub trait CheckFetchFamily<P, E> {
    type Checker: DataChecker;
}

impl<P, E, C> CheckFetchFamily<P, E> for C
where
    C: DataChecker,
{
    type Checker = C;
}

impl<Pre, C, E> ToInner for CheckExtract<Pre, C, E>
where
    C: CheckFetchFamily<Pre, E> + Sized,
{
    type Inner = <C::Checker as DataChecker>::Checked;

    fn to_inner(self) -> Self::Inner { self.0 }
}

pub type JsonCheckExtract<C, E> =
    CheckExtract<Json<<C as DataChecker>::Unchecked>, C, E>;

pub type FormCheckExtract<C, E> =
    CheckExtract<Form<<C as DataChecker>::Unchecked>, C, E>;

pub type PathCheckExtract<C, E> =
    CheckExtract<Path<<C as DataChecker>::Unchecked>, C, E>;

pub type QueryCheckExtract<C, E> =
    CheckExtract<Query<<C as DataChecker>::Unchecked>, C, E>;
