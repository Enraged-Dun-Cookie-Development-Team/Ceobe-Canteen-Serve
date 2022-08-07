use std::convert::Infallible;

use async_trait::async_trait;
use axum::{
    body::Body,
    extract::{FromRequest, OriginalUri, RequestParts},
};
use http::{
    header::{ETAG, LAST_MODIFIED},
    method::Method,
    StatusCode,
};
use resp_result::{ExtraFlag, ExtraFlags};

use crate::{
    cache_ctrl::CacheInfo,
    error::VerifyResult,
    headers::{self, ControlHeaders},
    time_format::{from_request_head, to_request_header},
    traits::{CacheState, ModifyState},
};

pub struct CacheVerify {
    ctrl_header: ControlHeaders,
    pub cache_info: CacheInfo,
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
                    cache_info: Default::default(),
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
                    cache_info: CacheInfo {
                        content_local: Some(uri),
                        ..Default::default()
                    },
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
        let last_modify = data
            .get_last_modify_time()
            .and_then(|v| to_request_header(&v).ok());

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
                    Ok(CacheState::NotModify) => {
                        (
                            None,
                            ExtraFlag::empty_body()
                                + ExtraFlag::status(StatusCode::NOT_MODIFIED),
                        )
                    }
                    Err(v) | Ok(CacheState::Update(v)) => {
                        (Some(v), ().into())
                    }
                }
            }
            ControlHeaders::None => (Some(data), ().into()),
        };
        extra_flags = extra_flags
        // entity tag
        + ExtraFlag::insert_header(ETAG, format!("\"{tag}\""))
        + &self.cache_info;

        if let Some(last_modify) = last_modify {
            // last modify
            extra_flags = extra_flags
                + ExtraFlag::insert_header(LAST_MODIFIED, last_modify)
        }

        Ok((data, extra_flags))
    }
}
