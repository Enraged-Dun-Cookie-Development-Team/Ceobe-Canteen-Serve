#[allow(unused_imports)]
use std::str::FromStr;

use http::{header::HeaderName, HeaderValue, StatusCode};

use crate::{get_config, resp_error::RespError};

use super::RespResult;
#[allow(dead_code)]
#[inline]
fn prepare_respond<T, E>(
    r: RespResult<T, E>,
) -> Result<(Vec<u8>, StatusCode, Option<(HeaderName, HeaderValue)>), serde_json::Error>
where
    T: serde::Serialize,
    E: RespError,
{
    #[allow(unused_variables)]
    let cfg = &get_config().resp;

    let vec = serde_json::to_vec(&r);
    let body = match vec {
        Ok(body) => body,
        Err(e) => {
            #[cfg(feature = "log")]
            logger::error!("RespResult 响应出现异常 : {}", e);
            Err(e)?;
            unreachable!()
        }
    };

    let status = match r {
        RespResult::Success(_) => {
            #[cfg(feature = "log")]
            logger::info!("RespResult 接管的 成功 响应",);
            StatusCode::OK
        }
        RespResult::Err(ref e) => {
            #[cfg(feature = "log")]
            logger::info!(
                "RespResult 接管的 异常 响应 | {} => {}",
                std::any::type_name::<E>(),
                e.description()
            );
            e.http_code()
        }
    };
    #[cfg(feature = "extra-code")]
    let r_header = {
        match &r {
            RespResult::Success(_) => None,
            RespResult::Err(e) => {
                if let Some(n) = cfg.extra_code {
                    Some((
                        HeaderName::from_str(n).expect("Bad HeaderName"),
                        HeaderValue::from_str(&e.extra_code().to_string())
                            .expect("Bad HeaderValue"),
                    ))
                } else {
                    None
                }
            }
        }
    };
    #[cfg(not(feature = "extra-code"))]
    let r_header = None;

    #[cfg(feature = "log")]
    logger::info!(
        "RespResult 响应 准备构造 Status :{} BodySize: {}",
        status,
        body.len()
    );
    Ok((body, status, r_header))
}

#[allow(dead_code)]
static JSON_TYPE: &str = "application/json";

#[cfg(feature = "for-axum")]
impl<T, E> axum::response::IntoResponse for RespResult<T, E>
where
    T: serde::Serialize,
    E: RespError,
{
    #[inline]
    fn into_response(self) -> axum::response::Response {
        match prepare_respond(self) {
            Ok((body, status, eh)) => match eh {
                None => axum::response::Response::builder()
                    .status(status)
                    .header(http::header::CONTENT_TYPE, JSON_TYPE)
                    .body(axum::body::boxed(axum::body::Full::from(body))),
                Some((k, v)) => axum::response::Response::builder()
                    .status(status)
                    .header(http::header::CONTENT_TYPE, JSON_TYPE)
                    .header(k, v)
                    .body(axum::body::boxed(axum::body::Full::from(body))),
            },
            Err(e) => axum::response::Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(axum::body::boxed(axum::body::Full::from(e.to_string()))),
        }
        .expect("RespResult 构造响应时发生异常")
    }
}

#[cfg(feature = "for-actix")]
impl<T, E> actix_web::Responder for RespResult<T, E>
where
    T: serde::Serialize,
    E: RespError,
{
    type Body = actix_web::body::BoxBody;
    
    #[inline]
    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        match prepare_respond(self) {
            Ok((body, status, eh)) => match eh {
                None => actix_web::HttpResponse::build(status)
                    .content_type(JSON_TYPE)
                    .body(body),
                Some((k, v)) => actix_web::HttpResponse::build(status)
                    .content_type(JSON_TYPE)
                    .insert_header((k, v))
                    .body(body),
            },
            Err(e) => actix_web::HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                .body(e.to_string()),
        }
    }
}
