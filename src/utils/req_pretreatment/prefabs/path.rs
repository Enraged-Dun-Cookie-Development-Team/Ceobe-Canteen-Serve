use std::marker::PhantomData;

use actix_web::{web, FromRequest};
use futures::future::{ready, Ready};
use serde::de::DeserializeOwned;
use status_err::ErrPrefix;

use crate::utils::req_pretreatment::Pretreatment;

pub struct PathValue<P>(PhantomData<P>);

impl<P: DeserializeOwned> Pretreatment for PathValue<P> {
    type Fut = Ready<Result<Self::Resp, Self::Err>>;

    type Resp = P;

    type Err = PathError;

    fn call<'r>(
        req: &'r actix_web::HttpRequest,
        payload: &'r mut actix_http::Payload,
    ) -> Self::Fut {
        let fut = web::Path::<P>::from_request(req, payload)
            .into_inner()
            .map(|d| d.into_inner())
            .map_err(|e| PathError(e));
        ready(fut)
    }
}

crate::error_generate!(pub PathError(actix_web::Error));
status_err::status_error!(PathError[ErrPrefix::CHECKER,0005]);
