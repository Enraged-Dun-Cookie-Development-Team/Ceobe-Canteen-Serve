use std::{
    fmt::{Debug, Formatter},
    ops::Deref,
};

use axum::{async_trait, extract::FromRequestParts};
use axum_resp_result::{Nil,  RespResult};
use persistence::{admin, operate::Parts};
use status_err::{resp_error_impl, status_error, ErrPrefix, HttpCode};

#[derive(Clone)]
pub struct AuthorizedUser(pub admin::models::Model);

#[async_trait]
impl<S> FromRequestParts<S> for AuthorizedUser {
    type Rejection = RespResult<Nil, NoAuthorizeLayerError>;

    async fn from_request_parts(
        parts: &mut Parts, _: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .remove::<Self>()
            .ok_or(NoAuthorizeLayerError)
            .map_err(RespResult::err)
    }
}

impl Debug for AuthorizedUser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthorizeInfo")
            .field("name", &self.0.username)
            .field("auth_level", &self.0.auth)
            .field("token_version", &self.0.num_pwd_change)
            .finish()
    }
}

impl Deref for AuthorizedUser {
    type Target = admin::models::Model;

    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, thiserror::Error)]
#[error("缺少Authorize鉴权中间件")]
pub struct NoAuthorizeLayerError;

resp_error_impl!(NoAuthorizeLayerError);

status_error!(
    NoAuthorizeLayerError
    [
        ErrPrefix::UNAUTHORIZED,
        0x000A: HttpCode::INTERNAL_SERVER_ERROR
    ]
);
