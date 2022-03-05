use futures::future::ok;
use futures_util::future::Ready;
use std::convert::Infallible;

use crate::utils::req_pretreatment::Pretreatment;

pub struct Null;

impl Pretreatment for Null {
    type Fut = Ready<Result<Self::Resp, Self::Err>>;

    type Resp = ();

    type Err = Infallible;

    fn call<'r>(
        _req: &'r actix_web::HttpRequest,
        _payload: &'r mut actix_http::Payload,
    ) -> Self::Fut {
        ok(())
    }
}
