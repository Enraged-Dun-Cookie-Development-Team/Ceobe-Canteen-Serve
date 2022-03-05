use actix_service::{Service, Transform};

use futures::future::{ok, Ready};

use futures::Future;

pub struct BenchMark<S> {
    service: S,
}

impl<S> actix_service::Service for BenchMark<S>
where
    S: Service,
    S::Future: 'static,
{
    type Request = S::Request;

    type Response = S::Response;

    type Error = S::Error;

    type Future = impl Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        let task = self.service.call(req);
        async move {
            let req_income = std::time::SystemTime::now();
            let res = task.await;
            let resp_outcome = std::time::SystemTime::now();

            let using_time = resp_outcome.duration_since(req_income).unwrap();

            log::info!("Handling Request Done Using [{}ms]", using_time.as_millis());

            res
        }
    }
}

pub struct BenchMarkFactor;

impl<S> Transform<S> for BenchMarkFactor
where
    S: Service,
    S::Future: 'static,
{
    type Request = S::Request;

    type Response = S::Response;

    type Error = S::Error;

    type Transform = BenchMark<S>;

    type InitError = ();

    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(BenchMark { service })
    }
}
