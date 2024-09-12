use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum_resp_result::{Nil, RespResult};
use http::request::Parts;
pub use layer::AuthorizeLayer;
use persistence::admin;

pub use self::error::AuthorizeError;

mod error;
mod layer;
mod service;

#[derive(Clone)]
pub struct AuthorizeInfo(pub admin::models::Model);

impl std::fmt::Debug for AuthorizeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthorizeInfo")
            .field("name", &self.0.username)
            .field("auth_level", &self.0.auth)
            .field("token_version", &self.0.num_pwd_change)
            .finish()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthorizeInfo
where
    S: Send + Sync,
{
    type Rejection = RespResult<Nil, AuthorizeError>;

    async fn from_request_parts(
        parts: &mut Parts, _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(parts
            .extensions
            .remove::<Self>()
            .ok_or(AuthorizeError::NoAuthorizeLayer)
            .map_err(RespResult::err)?)
    }
}
