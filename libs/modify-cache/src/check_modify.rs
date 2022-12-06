use std::convert::Infallible;

use async_trait::async_trait;
use axum::extract::{FromRequestParts, OriginalUri};
use http::{
    header::{CONTENT_TYPE, ETAG, LAST_MODIFIED},
    method::Method,
    request::Parts,
    StatusCode,
};
use resp_result::{ExtraFlag, ExtraFlags};
use tracing::warn;

use crate::{
    cache_ctrl::CacheHeaders,
    error::VerifyResult,
    headers::{self, ControlHeaders},
    time_format::{from_request_head, to_request_header},
    traits::{CacheState, ModifyState},
};

pub struct CheckModify {
    ctrl_header: ControlHeaders,
    pub cache_headers: CacheHeaders,
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for CheckModify {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts, state: &S,
    ) -> Result<Self, Self::Rejection> {
        let OriginalUri(uri) =
            OriginalUri::from_request_parts(parts, state).await?;

        // if not get or head , default none;
        Ok(
            if parts.method != Method::GET && parts.method != Method::HEAD {
                warn!(request.method = %parts.method, "Skipping");
                Self {
                    ctrl_header: ControlHeaders::None,
                    cache_headers: Default::default(),
                }
            } else {
                let header = &parts.headers;
                let ctrl_header = header
                    .get(http::header::IF_NONE_MATCH)
                    .and_then(|v| v.to_str().ok())
                    .filter(|v| v != &"*")
                    .and_then(|v| headers::etag(v).ok())
                    .filter(|(e, _)| e.is_empty())
                    .map(|(_e, v)| v)
                    .map(ControlHeaders::IfNoneMatch)
                    .or_else(|| {
                        header
                            .get(http::header::IF_MODIFIED_SINCE)
                            .and_then(|v| from_request_head(v).ok())
                            .map(ControlHeaders::IfModifySince)
                    })
                    .unwrap_or(ControlHeaders::None);
                Self {
                    ctrl_header,
                    cache_headers: CacheHeaders {
                        content_local: Some(uri),
                        ..Default::default()
                    },
                }
            },
        )
    }
}

impl CheckModify {
    pub fn check_modify<T: ModifyState>(
        &self, data: T,
    ) -> VerifyResult<(Option<T>, ExtraFlags)> {
        let tag = data.get_entity_tag()?;
        let last_modify = data
            .get_last_modify_time()
            .and_then(|v| to_request_header(&v).ok());

        let (data, mut extra_flags) = match &self.ctrl_header {
            ControlHeaders::IfNoneMatch(tags) => {
                match data.verify_entity_tag(tags, &tag)? {
                    CacheState::NotModify => (
                        None,
                        ExtraFlag::empty_body()
                            + ExtraFlag::status(StatusCode::NOT_MODIFIED)
                            + ExtraFlag::remove_header(CONTENT_TYPE),
                    ),
                    CacheState::Update(v) => (Some(v), ().into()),
                }
            }
            ControlHeaders::IfModifySince(date_time) => {
                match data.verify_modify(date_time) {
                    Ok(CacheState::NotModify) => (
                        None,
                        ExtraFlag::empty_body()
                            + ExtraFlag::remove_header(CONTENT_TYPE)
                            + ExtraFlag::status(StatusCode::NOT_MODIFIED),
                    ),
                    Err(v) | Ok(CacheState::Update(v)) => {
                        (Some(v), ().into())
                    }
                }
            }
            ControlHeaders::None => (Some(data), ().into()),
        };
        // entity tag
        extra_flags += ExtraFlag::insert_header(ETAG, format!("\"{tag}\""));
        extra_flags += &self.cache_headers;

        if let Some(last_modify) = last_modify {
            // last modify
            extra_flags +=
                ExtraFlag::insert_header(LAST_MODIFIED, last_modify)
        }

        Ok((data, extra_flags))
    }
}
