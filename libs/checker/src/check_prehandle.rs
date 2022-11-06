use std::marker::PhantomData;

use crate::{Checker as DataChecker, LiteArgs};

use axum::{
    extract::{FromRequest, Path, Query, RequestParts},
    Form, Json,
};
use resp_result::{resp_try, Nil, RespError, RespResult};

pub struct CheckExtract<Previous, C, E>(
    pub C::Checked,
    pub PhantomData<(Previous, E)>,
)
where
    Previous: UncheckFetcher<Uncheck = C::Unchecked>,
    C: DataChecker,
    C::Unchecked: Send,
    C::Args: LiteArgs + Send,
    C::Fut: Send,
    E: From<C::Err>,
    E: RespError;

impl<B, Previous, C, E> FromRequest<B> for CheckExtract<Previous, C, E>
where
    B: Send,
    Previous: FromRequest<B> + UncheckFetcher<Uncheck = C::Unchecked>,
    C: DataChecker,
    C::Unchecked: Send,
    C::Args: LiteArgs + Send,
    C::Fut: Send,
    E: From<Previous::Rejection>,
    E: From<C::Err>,
    E: RespError,
{
    type Rejection = RespResult<Nil, E>;

    fn from_request<'life0, 'async_trait>(
        req: &'life0 mut RequestParts<B>,
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
            match resp_try::<_, _, E>(async {
                let previous = Previous::from_request(req).await?.fetch();
                let checked = C::check(LiteArgs::get_arg(), previous).await?;
                Ok(Self(checked, PhantomData))
            })
            .await
            {
                RespResult::Success(this) => Ok(this),
                RespResult::Err(err) => Err(RespResult::Err(err)),
            }
        })
    }
}

pub trait UncheckFetcher {
    type Uncheck;
    fn fetch(self) -> Self::Uncheck;
}

impl<T> UncheckFetcher for Json<T> {
    type Uncheck = T;

    fn fetch(self) -> Self::Uncheck {
        self.0
    }
}

pub type JsonCheckExtract<C, E> =
    CheckExtract<Json<<C as DataChecker>::Unchecked>, C, E>;

impl<T> UncheckFetcher for Form<T> {
    type Uncheck = T;

    fn fetch(self) -> Self::Uncheck {
        self.0
    }
}

pub type FormCheckExtract<C, E> =
    CheckExtract<Form<<C as DataChecker>::Unchecked>, C, E>;

impl<T> UncheckFetcher for Path<T> {
    type Uncheck = T;

    fn fetch(self) -> Self::Uncheck {
        self.0
    }
}

pub type PathCheckExtract<C, E> =
    CheckExtract<Path<<C as DataChecker>::Unchecked>, C, E>;

impl<T> UncheckFetcher for Query<T> {
    type Uncheck = T;

    fn fetch(self) -> Self::Uncheck {
        self.0
    }
}

pub type QueryCheckExtract<C, E> =
    CheckExtract<Query<<C as DataChecker>::Unchecked>, C, E>;
