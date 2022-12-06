use core::marker::Send;

use axum::{
    extract::{FromRequest, FromRequestParts, Path, Query},
    http::{request::Parts, Request},
    Form, Json,
};
use resp_result::{Nil, RespError, RespResult, ToInner};

use crate::{Checker as DataChecker, LiteArgs};

pub struct CheckExtract<Previous, C, E>(
    pub <C::Checker as DataChecker>::Checked,
)
where
    Previous: ToInner<Inner = <C::Checker as DataChecker>::Unchecked>,
    C: CheckFetchFamily<Previous, E>,
    <C::Checker as DataChecker>::Unchecked: Send,
    <C::Checker as DataChecker>::Args: LiteArgs + Send,
    <C::Checker as DataChecker>::Fut: Send,
    E: From<<C::Checker as DataChecker>::Err>,
    E: RespError;

impl<S, Previous, C, E> FromRequestParts<S> for CheckExtract<Previous, C, E>
where
    S: Sync + Send,
    // 前置提取
    Previous: ToInner<Inner = <C::Checker as DataChecker>::Unchecked>,
    Previous: FromRequestParts<S>,
    // 检查器
    C: CheckFetchFamily<Previous, E>,
    <C::Checker as DataChecker>::Unchecked: Send,
    <C::Checker as DataChecker>::Args: LiteArgs + Send,
    <C::Checker as DataChecker>::Fut: Send,
    // 异常映射
    E: From<Previous::Rejection>,
    E: From<<C::Checker as DataChecker>::Err>,
    E: RespError,
{
    type Rejection = RespResult<Nil, E>;

    fn from_request_parts<'life0, 'life1, 'async_trait>(
        parts: &'life0 mut Parts, state: &'life1 S,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<Self, Self::Rejection>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async {
            async {
                let previous = Previous::from_request_parts(parts, state)
                    .await?
                    .to_inner();
                let checked =
                    C::Checker::check(LiteArgs::get_arg(), previous).await?;
                Ok::<_, E>(Self(checked))
            }
            .await
            .map_err(RespResult::Err)
        })
    }
}

impl<S, B, Previous, C, E> FromRequest<S, B> for CheckExtract<Previous, C, E>
where
    B: Send + 'static,
    S: Sync + Send,
    Previous: ToInner<Inner = <C::Checker as DataChecker>::Unchecked>,
    Previous: FromRequest<S, B>,
    // 检查器
    C: CheckFetchFamily<Previous, E>,
    <C::Checker as DataChecker>::Unchecked: Send,
    <C::Checker as DataChecker>::Args: LiteArgs + Send,
    <C::Checker as DataChecker>::Fut: Send,
    // 异常映射
    E: From<Previous::Rejection>,
    E: From<<C::Checker as DataChecker>::Err>,
    E: RespError,
{
    type Rejection = RespResult<Nil, E>;

    fn from_request<'life0, 'async_trait>(
        req: Request<B>, state: &'life0 S,
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
                    Previous::from_request(req, state).await?.to_inner();
                let checked =
                    C::Checker::check(LiteArgs::get_arg(), previous).await?;
                Ok(Self(checked))
            }
            .await
            .map_err(RespResult::Err)
        })
    }
}

pub trait CheckFetchFamily<P: ToInner, E> {
    type Checker: DataChecker;
}

impl<P, E, C> CheckFetchFamily<P, E> for C
where
    P: ToInner,
    C: DataChecker,
{
    type Checker = C;
}

pub type JsonCheckExtract<C, E> =
    CheckExtract<Json<<C as DataChecker>::Unchecked>, C, E>;

pub type FormCheckExtract<C, E> =
    CheckExtract<Form<<C as DataChecker>::Unchecked>, C, E>;

pub type PathCheckExtract<C, E> =
    CheckExtract<Path<<C as DataChecker>::Unchecked>, C, E>;

pub type QueryCheckExtract<C, E> =
    CheckExtract<Query<<C as DataChecker>::Unchecked>, C, E>;
