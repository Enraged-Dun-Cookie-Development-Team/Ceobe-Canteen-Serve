use std::{
    fmt::Debug,
    ops::Deref,
};

use axum::{async_trait, extract::FromRequestParts};
use axum_resp_result::{Nil, RespResult};

use persistence::operate::Parts;
use status_err::{ErrPrefix, HttpCode, resp_error_impl, status_error};

#[derive(Clone,Debug)]
pub struct AuthorizedUser<T:Send + Sync + Clone+'static>(pub T);

#[async_trait]
impl<S,T> FromRequestParts<S> for AuthorizedUser<T>
where T:Send +Sync +Clone+'static
{
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



impl<T:Send +Sync +Clone +'static> Deref for AuthorizedUser<T> {
    type Target = T;

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
