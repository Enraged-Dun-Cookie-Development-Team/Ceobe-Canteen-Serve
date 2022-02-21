use std::io::Cursor;

use futures_util::future::{err, ok, Ready};

use rocket_::{
    http::{ContentType, Status},
    response::{self, Responder},
    Request,
};

use crate::{IntoSerde, RResult};

impl<'r, 'o, T, E> Responder<'r, 'o> for RResult<T, E>
where
    'o: 'r,
    T: for<'s> IntoSerde<'s>,
    E: std::error::Error,
{
    fn respond_to(self, _request: &'r Request<'_>) -> response::Result<'o> {
        let t = serde_json::to_vec(&self).or_else(|_e| Err(Status::InternalServerError))?;
        let status = match self {
            RResult::Success(_) => Status::Ok.code,
            RResult::Error(s, _) => s.as_u16(),
        };

        #[cfg(feature = "logger")]
        log::info!(
            "Respond by RRsult | status: {}, content-size: {}",
            status,
            t.len()
        );

        rocket_::Response::build()
            .header(ContentType::JSON)
            .sized_body(t.len(), Cursor::new(t))
            .status(Status::new(status))
            .ok()
    }
}

impl<T, E> actix_web::Responder for RResult<T, E>
where
    T: for<'s> IntoSerde<'s>,
    E: std::error::Error,
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
            RResult::Error(s, _) => s,
        };
        #[cfg(feature = "logger")]
        log::info!(
            "Respond by RRsult | status: {}, content-size: {}",
            status,
            body.len()
        );

        ok(actix_web::HttpResponse::build(status)
            .content_type("application/json")
            .body(body))
    }
}
