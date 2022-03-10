use std::borrow::Cow;

use actix_http::body::BoxBody;
use actix_web::HttpResponse;

use crate::{IntoSerde, RResult};

impl<T, E> actix_web::Responder for RResult<T, E>
where
T: for<'s> IntoSerde<'s>,
E: status_err::StatusErr,
{
    type Body=BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) ->HttpResponse<Self::Body>   {      
        let status = match self {
            RResult::Success(_) => http::StatusCode::OK,
            RResult::Error(ref s) => s.http_code(),
        };

        let head_status = match self {
            RResult::Success(_) => Cow::Borrowed("00000"),
            RResult::Error(ref e) => format!("{}{}",e.prefix(),e.code()).into(),
        };

        #[cfg(feature = "logger")]
        log::info!(
            "Respond by RRsult | status: {}, head-status: {}",
            status,
            head_status
        );

        actix_web::HttpResponse::build(status)
            .insert_header(("Status-Code", &*head_status))
            .json(self)
    }

}
