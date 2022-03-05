pub mod prefabs;
pub mod db_operate;
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use futures::Future;
use std::ops::{Deref, DerefMut};

/// 用于预处理请求头的trait
pub trait Pretreatment {
    type Fut: Future<Output = Result<Self::Resp, Self::Err>>;
    type Resp;
    type Err: Into<actix_http::Error>;

    fn call<'r>(req: &'r HttpRequest, payload: &'r mut Payload) -> Self::Fut;
}

pub struct ReqPretreatment<Pre>(Pre::Resp)
where
    Pre: Pretreatment;

impl<Pre> ReqPretreatment<Pre>
where
    Pre: Pretreatment,
{
    #[inline]
    pub fn into_inner(self) -> Pre::Resp {
        self.0
    }
}

impl<Pre> DerefMut for ReqPretreatment<Pre>
where
    Pre: Pretreatment,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<Pre> Deref for ReqPretreatment<Pre>
where
    Pre: Pretreatment,
{
    type Target = Pre::Resp;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Pre> FromRequest for ReqPretreatment<Pre>
where
    Pre: Pretreatment,
{
    type Error = Pre::Err;

    type Future = impl Future<Output = Result<Self, Self::Error>>;

    type Config = ();
    #[inline]
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let task = Pre::call(req, payload);

        async move {
            let inner = task.await?;
            Ok(Self(inner))
        }
    }
}

impl<R:FromRequest> Pretreatment for R   {
    type Fut=R::Future;

    type Resp=R;

    type Err=R::Error;

    fn call<'r>(req: &'r HttpRequest, payload: &'r mut Payload) -> Self::Fut {
        R::from_request(req, payload)
    }
}