use std::marker::PhantomData;

use futures::Future;

use crate::utils::req_pretreatment::Pretreatment;

pub struct MapErr<P, E>(PhantomData<P>, PhantomData<E>);

impl<P, E> Pretreatment for MapErr<P, E>
where
    P: Pretreatment,
    P::Err: Into<E>,
{
    type Fut = impl Future<Output = Result<Self::Resp, Self::Err>>;

    type Resp = P::Resp;

    type Err = E;
    #[inline]
    fn call<'r>(
        req: &'r actix_web::HttpRequest,
        payload: &'r mut actix_http::Payload,
    ) -> Self::Fut {
        let task = P::call(req, payload);
        async move {
            let resp = task.await.map_err(|pe| pe.into())?;
            Ok(resp)
        }
    }
}
