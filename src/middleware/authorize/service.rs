use std::marker::PhantomData;

use axum::{
    body::{Body, BoxBody},
    extract::{FromRequest, RequestParts},
    response::{IntoResponse, Response},
};
use futures::future::BoxFuture;
use http::Request;
use orm_migrate::{
    sql_connection::SqlConnect,
    sql_models::admin_user::operate::{OperateError, UserSqlOperate},
};
use resp_result::RespResult;
use tap::Tap;
use tower_http::auth::AsyncAuthorizeRequest;
use tracing::{info, warn, Instrument};
use tracing_unwrap::{OptionExt, ResultExt};

use super::{error::AuthorizeError, AuthorizeInfo};
use crate::utils::user_authorize::{
    auth_level::{AuthLevelVerify, UnacceptableAuthorizationLevelError},
    config::get_authorize_information,
    decrypt_token, User,
};
pub struct AdminAuthorize<L>(PhantomData<L>);

impl<L> Clone for AdminAuthorize<L> {
    fn clone(&self) -> Self { Self::default() }
}

impl<L> Default for AdminAuthorize<L> {
    fn default() -> Self { Self(PhantomData) }
}

impl<L: AuthLevelVerify> AsyncAuthorizeRequest<Body> for AdminAuthorize<L> {
    type Future = BoxFuture<'static, Result<Request<Body>, Response>>;
    type RequestBody = Body;
    type ResponseBody = BoxBody;

    fn authorize(&mut self, request: Request<Body>) -> Self::Future {
        Box::pin(async move {
            let result = 'auth: {
                let Some(token) = get_authorize_information(&request) else{
                    break 'auth Err(AuthorizeError::TokenNotFound)
                };

                let User { id, num_pwd_change } = match decrypt_token(&token).map_err(AuthorizeError::from){
                    Ok(user) => user,
                    Err(err) => break 'auth Err(err)
                };

                let mut part = RequestParts::new(request);
                let db = SqlConnect::from_request(&mut part).await.unwrap();

                let req = part
                    .try_into_request()
                    .expect_or_log("Sql Data using Request Body");

                let user = match UserSqlOperate::find_user_with_version_verify(
                    &db,
                    id,
                    num_pwd_change,
                    |user| user,
                    || AuthorizeError::TOkenInvalid,
                )
                .await
                .map_err(|err| {
                    let OperateError::UserNotExist = err else {
                        return AuthorizeError::from(err);
                    };
                    AuthorizeError::TokenInfoNotFound
                })
                .and_then(|v| v){
                    Ok(user) => user,
                    Err(err)=>break 'auth Err(err)
                };

                let verify@true = L::verify(&user.auth) else {
                    warn!(
                        admin.name = user.username,
                        admin.auth_level = ?user.auth,
                        admin.has_permission = false,
                        auth_name = L::auth_name(),
                        uri = %req.uri()
                    );
                    break 'auth Err(AuthorizeError::AuthorizeLevel(UnacceptableAuthorizationLevelError::new(L::auth_name())));
                };

                info!(
                    admin.name = user.username,
                    admin.auth_level = ?user.auth,
                    permission.accept = verify
                );

                Ok(req.tap_mut(|req| {
                    req.extensions_mut()
                        .insert(AuthorizeInfo(user))
                        .expect_none_or_log("Authorize Layer Exist")
                }))
            }.map_err(|err|RespResult::<(), _>::Err(err).into_response());

            result
        }.instrument(tracing::info_span!("authorization")))
    }
}
