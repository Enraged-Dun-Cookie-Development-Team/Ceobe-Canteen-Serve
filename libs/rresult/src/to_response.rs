use futures_util::future::{err, ok, Ready};

use crate::{IntoSerde, RResult};

impl<T, E> actix_web::Responder for RResult<T, E>
where
    T: for<'s> IntoSerde<'s>,
    E: crate::ErrorCode,
{
    type Error = actix_http::error::Error;

    type Future = Ready<Result<actix_web::HttpResponse, <Self as actix_web::Responder>::Error>>;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> Self::Future {
        let data = serde_json::to_vec(&self);
        let body = match data {
            Ok(b) => b,
            Err(e) => return err(e.into()),
        };

        let status = match self {
            RResult::Success(_) => http::StatusCode::OK,
            RResult::Error(ref s) => s.http_code(),
        };

        let head_status = match self {
            RResult::Success(_) => 2000,
            RResult::Error(ref e) => e.code(),
        };

        #[cfg(feature = "logger")]
        log::info!(
            "Respond by RRsult | status: {}, head-status: {}, content-size: {}",
            status,
            head_status,
            body.len()
        );

        ok(actix_web::HttpResponse::build(status)
            .content_type("application/json")
            .set_header("Status-Code", head_status.to_string())
            .body(body))
    }
}
