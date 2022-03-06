use actix_web::{web, FromRequest};
use futures::Future;

use crate::mansion::error::{self, UnknownId};

pub struct SplitId;

impl super::Pretreatment for SplitId {
    type Fut = impl Future<Output = Result<Self::Resp, Self::Err>>;

    type Resp = (u32, u32);

    type Err = error::MansionError;

    fn call<'r>(
        req: &'r actix_web::HttpRequest,
        payload: &'r mut actix_http::Payload,
    ) -> Self::Fut {
        let task = web::Path::<MansionId>::from_request(req, payload);
        async move {
            let s = task.await?.0.id;
            let mut split = s.split('.');

            let first = split.next().ok_or(UnknownId)?;
            let first = first.trim().parse::<u32>().map_err(|_e| UnknownId)?;
            let second = split.next().unwrap_or("0");
            let second = second.trim().parse::<u32>().map_err(|_e| UnknownId)?;

            Ok((first, second))
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct MansionId {
    id: String,
}
