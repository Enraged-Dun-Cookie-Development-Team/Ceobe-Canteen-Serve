use std::convert::Infallible;

use futures::future::ok;
use futures_util::future::Ready;

use crate::utils::req_pretreatment::Pretreatment;

pub struct Null;

impl Pretreatment for Null {
    type Err = Infallible;
    type Fut = Ready<Result<Self::Resp, Self::Err>>;
    type Resp = ();

    #[inline]
    fn call<'r>(
        _req: &'r actix_web::HttpRequest,
        _payload: &'r mut actix_http::Payload,
    ) -> Self::Fut {
        ok(())
    }
}
