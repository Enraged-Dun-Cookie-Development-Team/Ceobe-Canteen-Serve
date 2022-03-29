use std::marker::PhantomData;

use actix_web::{web, FromRequest};
use futures::Future;
use serde::Deserialize;
use status_err::ErrPrefix;

use crate::{error_generate, utils::req_pretreatment::Pretreatment};

pub struct Json<T>(PhantomData<T>);

impl<T> Pretreatment for Json<T>
where
    T: for<'de> Deserialize<'de> + 'static,
{
    type Err = JsonError;
    type Resp = T;

    type Fut = impl Future<Output = Result<T, Self::Err>>;

    #[inline]
    fn call<'r>(
        req: &'r actix_web::HttpRequest, payload: &'r mut actix_http::Payload,
    ) -> Self::Fut {
        let task = web::Json::<T>::from_request(req, payload);
        async move { task.await.map(|j| j.into_inner()).map_err(|e| JsonError(e)) }
    }
}

error_generate!(pub JsonError(actix_web::Error));

status_err::status_error!(JsonError[ErrPrefix::CHECKER,0004]);
