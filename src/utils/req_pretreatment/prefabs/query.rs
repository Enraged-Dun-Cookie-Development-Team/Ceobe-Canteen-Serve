use std::marker::PhantomData;

use actix_web::web;
use serde::Deserialize;

use crate::utils::req_pretreatment::Pretreatment;

pub struct Query<T>(PhantomData<T>)
where
    T: for<'de> Deserialize<'de>;

impl<T> Pretreatment for Query<T>
where
    T: for<'de> Deserialize<'de>,
{
    type Err = actix_web::error::QueryPayloadError;
    type Fut = futures_util::future::Ready<Result<Self::Resp, Self::Err>>;
    type Resp = T;

    fn call<'r>(
        req: &'r actix_web::HttpRequest, _: &'r mut actix_http::Payload,
    ) -> Self::Fut {
        let query = req.query_string();
        let resp = web::Query::<T>::from_query(query).map(|q| q.into_inner());
        futures_util::future::ready(resp)
    }
}
