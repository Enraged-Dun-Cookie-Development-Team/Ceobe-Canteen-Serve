use std::marker::PhantomData;

use futures::Future;

use crate::utils::req_pretreatment::Pretreatment;

pub struct Pair<L, R>(PhantomData<L>, PhantomData<R>);

pub enum PairErr<Le, Re> {
    Left(Le),
    Right(Re),
}

impl<Le, Re> Into<actix_http::Error> for PairErr<Le, Re>
where
    Le: Into<actix_http::Error>,
    Re: Into<actix_http::Error>,
{
    fn into(self) -> actix_http::Error {
        match self {
            PairErr::Left(l) => l.into(),
            PairErr::Right(r) => r.into(),
        }
    }
}

impl<L, R> Pretreatment for Pair<L, R>
where
    L: Pretreatment,
    R: Pretreatment,
{
    type Fut = impl Future<Output = Result<Self::Resp, Self::Err>>;

    type Resp = (L::Resp, R::Resp);

    type Err = PairErr<L::Err, R::Err>;

    fn call<'r>(
        req: &'r actix_web::HttpRequest,
        payload: &'r mut actix_http::Payload,
    ) -> Self::Fut {
        let l_fut = L::call(req, payload);
        let r_fut = R::call(req, payload);
        async move {
            let l = l_fut.await.map_err(PairErr::Left)?;
            let r = r_fut.await.map_err(PairErr::Right)?;
            Ok((l, r))
        }
    }
}
