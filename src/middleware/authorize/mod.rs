mod layer;
use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts};
use orm_migrate::sql_models::admin_user::models::user;
use resp_result::{Nil, RespResult};

pub use self::error::AuthorizeError;

mod error;
mod service;

pub struct AuthorizeInfo(pub user::Model);

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
impl<B: Send> FromRequest<B> for AuthorizeInfo {
    type Rejection = RespResult<Nil, AuthorizeError>;

    async fn from_request(
        req: &mut RequestParts<B>,
    ) -> Result<Self, Self::Rejection> {
        let v = req
            .extensions_mut()
            .remove::<Self>()
            .ok_or(AuthorizeError::NoAuthorizeLayer)
            .map_err(RespResult::err)?;
        Ok(v)
    }
}

pub use layer::AuthorizeLayer;
pub use service::AdminAuthorize;
