use std::{convert::Infallible, time::Duration};

use async_trait::async_trait;
use axum::{
    body::Body,
    extract::{FromRequest, OriginalUri, RequestParts},
};
use http::{
    header::{CACHE_CONTROL, CONTENT_LOCATION, ETAG, LAST_MODIFIED, VARY},
    method::Method,
    uri::Uri,
    StatusCode,
};
use resp_result::{ExtraFlag, ExtraFlags};

use crate::{
    error::VerifyResult,
    headers::{self, ControlHeaders},
    time_format::{from_request_head, to_request_header},
    traits::{CacheState, ModifyState},
};

pub struct CacheVerify {
    ctrl_header: ControlHeaders,
    uri: Uri,
    pub enable_content_local: bool,
    pub enable_vary: bool,
    pub max_age: Duration,
    pub public_cache: bool,
}

#[async_trait]
impl FromRequest<Body> for CacheVerify {
    type Rejection = Infallible;

    async fn from_request(
        req: &mut RequestParts<Body>,
    ) -> Result<Self, Self::Rejection> {
        let OriginalUri(uri) = OriginalUri::from_request(req).await?;

        // if not get or head , default none;
        Ok(
            if req.method() != Method::GET && req.method() != Method::HEAD {
                log::warn!("不是`GET` 或者 `HEAD` 方法,不获取任何内容");
                Self {
                    ctrl_header: ControlHeaders::None,
                    uri,
                    enable_content_local: true,
                    enable_vary: true,
                    max_age: Duration::from_secs(28800),
                    public_cache: true,
                }
            }
            else {
                let header = req.headers();
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
                    uri,
                    enable_content_local: true,
                    enable_vary: true,
                    max_age: Duration::from_secs(28800),
                    public_cache: true,
                }
            },
        )
    }
}

impl CacheVerify {
    pub fn is_modify<T: ModifyState>(
        &self, data: T,
    ) -> VerifyResult<(Option<T>, ExtraFlags)> {
        let tag = data.get_entity_tag()?;
        let last_modify = to_request_header(&data.get_last_modify_time())?;

        let (data, mut extra_flags) = match &self.ctrl_header {
            ControlHeaders::IfNoneMatch(tags) => {
                match data.verify_entity_tag(tags)? {
                    CacheState::NotModify => {
                        (
                            None,
                            ExtraFlag::empty_body()
                                + ExtraFlag::status(StatusCode::NOT_MODIFIED),
                        )
                    }
                    CacheState::Update(v) => (Some(v), ().into()),
                }
            }
            ControlHeaders::IfModifySince(date_time) => {
                match data.verify_modify(date_time) {
                    CacheState::NotModify => {
                        (
                            None,
                            ExtraFlag::empty_body()
                                + ExtraFlag::status(StatusCode::NOT_MODIFIED),
                        )
                    }
                    CacheState::Update(v) => (Some(v), ().into()),
                }
            }
            ControlHeaders::None => (Some(data), ().into()),
        };
        extra_flags = extra_flags
        // entity tag
        + ExtraFlag::insert_header(ETAG, format!("\"{tag}\""))
        // last modify
        + ExtraFlag::insert_header(LAST_MODIFIED,last_modify);
        if self.enable_content_local {
            // local
            extra_flags = extra_flags
                + ExtraFlag::insert_header(
                    CONTENT_LOCATION,
                    &self.uri.to_string(),
                );
        }
        if self.enable_vary {
            // using cache with headers
            extra_flags = extra_flags
                + ExtraFlag::insert_header(VARY, "ETag, Last-Modified")
        }

        // cache config pub cache 12h
        extra_flags = extra_flags
            + ExtraFlag::insert_header(
                CACHE_CONTROL,
                format!(
                    "{0}, s-maxage={1}, max-age={1}",
                    if self.public_cache {
                        "public"
                    }
                    else {
                        "private"
                    },
                    self.max_age.as_secs()
                ),
            );
        Ok((data, extra_flags))
    }
}
