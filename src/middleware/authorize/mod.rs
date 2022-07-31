mod layer;
use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts};
use orm_migrate::sql_models::admin_user::models::user;
use resp_result::{Nil, RespResult};

pub use self::error::AuthorizeError;

mod error;
mod service;

pub struct AuthorizeInfo(pub user::Model);

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
pub use service::AuthorizeService;