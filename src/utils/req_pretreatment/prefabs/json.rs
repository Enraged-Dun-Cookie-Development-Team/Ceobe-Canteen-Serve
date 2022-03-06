use std::marker::PhantomData;

use actix_web::{web, FromRequest};
use futures::Future;
use serde::Deserialize;

use crate::utils::req_pretreatment::Pretreatment;

pub struct Json<T>(PhantomData<T>);

impl<T> Pretreatment for Json<T>
where
    T: for<'de> Deserialize<'de> + 'static,
{
    type Fut = impl Future<Output = Result<T, Self::Err>>;

    type Resp = T;

    type Err = <actix_web::web::Json<T> as FromRequest>::Error;

    #[inline]
    fn call<'r>(
        req: &'r actix_web::HttpRequest,
        payload: &'r mut actix_http::Payload,
    ) -> Self::Fut {
        let task = web::Json::<T>::from_request(req, payload);
        async move { task.await.map(|j| j.into_inner()) }
    }
}
