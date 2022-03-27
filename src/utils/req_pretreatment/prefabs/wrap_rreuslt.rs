use std::{convert::Infallible, marker::PhantomData};

use futures::Future;
use resp_result::RespResult;
// use rresult::RResult;
// use status_err::StatusErr;

use crate::utils::req_pretreatment::Pretreatment;

pub struct WrapRResult<P>(PhantomData<P>)
where
    P: Pretreatment,
    P::Err: resp_result::RespError;

impl<P> Pretreatment for WrapRResult<P>
where
    P: Pretreatment,
    P::Err: resp_result::RespError,
{
    type Fut = impl Future<Output = Result<Self::Resp, Self::Err>>;

    type Resp = RespResult<P::Resp, P::Err>;

    type Err = Infallible;
    #[inline]
    fn call<'r>(
        req: &'r actix_web::HttpRequest,
        payload: &'r mut actix_http::Payload,
    ) -> Self::Fut {
        let task = P::call(req, payload);
        async move {
            Ok(match task.await {
                Ok(data) => RespResult::ok(data),
                Err(err) => RespResult::err(err),
            })
        }
    }
}
