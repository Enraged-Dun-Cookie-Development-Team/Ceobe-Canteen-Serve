use std::convert::Infallible;

use async_trait::async_trait;
use axum::{
    body::Body,
    extract::{FromRequest, RequestParts},
};
use chrono::NaiveDateTime;
use resp_result::{ExtraFlag, ExtraFlags};

use crate::{
    error::VerifyResult,
    time_format::{from_request_head, to_request_header},
    traits::{CacheState, ModifyState},
};

pub struct CacheVerify {
    etag_raw: Option<String>,
    modify_time: Option<NaiveDateTime>,
}

#[async_trait]
impl FromRequest<Body> for CacheVerify {
    type Rejection = Infallible;

    async fn from_request(
        req: &mut RequestParts<Body>,
    ) -> Result<Self, Self::Rejection> {
        let header = req.headers();
        let etag = header
            .get(http::header::IF_NONE_MATCH)
            .and_then(|v| v.to_str().ok())
            //  might be * as None
            .filter(|v| v != &"*")
            .map(ToOwned::to_owned);
        let modify = header
            .get(http::header::IF_MODIFIED_SINCE)
            .and_then(|v| from_request_head(v).ok());

        Ok(Self {
            etag_raw: etag,
            modify_time: modify,
        })
    }
}

impl CacheVerify {
    pub fn is_modify<T: ModifyState>(
        &self, data: T,
    ) -> VerifyResult<(Option<T>, ExtraFlags)> {
        if let Some(etag) = &self.etag_raw {
            Ok(match data.verify_entity_tag(etag)? {
                CacheState::NotModify => {
                    (
                        None,
                        ExtraFlag::empty_body()
                            + ExtraFlag::status(
                                http::StatusCode::NOT_MODIFIED,
                            ),
                    )
                }
                CacheState::Update(data, etag) => {
                    (
                        Some(data),
                        ExtraFlag::insert_header(http::header::ETAG, etag)
                            .into(),
                    )
                }
            })
        }
        else if let Some(modify_time) = &self.modify_time {
            Ok(match data.verify_modify(modify_time) {
                CacheState::NotModify => {
                    (
                        None,
                        ExtraFlag::empty_body()
                            + ExtraFlag::status(
                                http::StatusCode::NOT_MODIFIED,
                            ),
                    )
                }
                CacheState::Update(data, time) => {
                    (
                        Some(data),
                        ExtraFlag::insert_header(
                            http::header::LAST_MODIFIED,
                            to_request_header(&time)?,
                        )
                        .into(),
                    )
                }
            })
        }
        else {
            Ok((Some(data), ().into()))
        }
    }
}
