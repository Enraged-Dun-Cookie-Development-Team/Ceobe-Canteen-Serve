use std::marker::PhantomData;

use axum::{
    body::Body,
    extract::FromRequestParts,
    response::{IntoResponse, Response},
};
use axum_resp_result::RespResult;
use persistence::{
    admin::{
        user::{OperateError, ToUser},
        ToAdmin,
    },
    help_crates::{futures::future::BoxFuture, tap::Tap},
    mysql::SqlDatabaseOperate,
};
use status_err::http::Request;
use tower_http::auth::AsyncAuthorizeRequest;
use tracing::{info, warn, Instrument};
use tracing_unwrap::OptionExt;

use crate::{
    authorized_user::AuthorizedUser,
    configure::get_authorize_information,
    middleware::error::AuthorizeError,
    payload::UserClaim,
    roles::{AuthorizationAccessDenyError, UserRoleVerify},
};

pub struct UserAuthorize<L>(PhantomData<L>);

impl<L> Clone for UserAuthorize<L> {
    fn clone(&self) -> Self { Self::default() }
}

impl<L> Default for UserAuthorize<L> {
    fn default() -> Self { Self(PhantomData) }
}

impl<L: UserRoleVerify> AsyncAuthorizeRequest<Body> for UserAuthorize<L> {
    type Future = BoxFuture<'static, Result<Request<Body>, Response>>;
    type RequestBody = Body;
    type ResponseBody = Body;

    fn authorize(&mut self, request: Request<Body>) -> Self::Future {
        Box::pin(
            async move {
                let result = 'auth: {
                    let Some(token) = get_authorize_information(&request)
                    else {
                        break 'auth Err(AuthorizeError::TokenNotFound);
                    };

                    let UserClaim {
                        id,
                        password_version,
                        ..
                    } = match UserClaim::from_jwt_token(&token)
                        .map_err(AuthorizeError::from)
                    {
                        Ok(user) => user,
                        Err(err) => break 'auth Err(err),
                    };

                    let (mut parts, body) = request.into_parts();
                    let db = SqlDatabaseOperate::from_request_parts(
                        &mut parts,
                        &(),
                    )
                    .await
                    .unwrap();
                    let req = Request::from_parts(parts, body);
                    let user = match db
                        .admin()
                        .user()
                        .find_user_with_version_verify(
                            id,
                            password_version,
                            |user| user,
                            || AuthorizeError::TokenInvalid,
                        )
                        .await
                        .map_err(|err| {
                            if let OperateError::UserNotExist = err {
                                AuthorizeError::TokenInfoNotFound
                            }
                            else {
                                AuthorizeError::from(err)
                            }
                        })
                        .and_then(|v| v)
                    {
                        Ok(user) => user,
                        Err(err) => break 'auth Err(err),
                    };

                    let verify @ true = L::access_verify(&user.auth)
                    else {
                        warn!(
                            admin.name = user.username,
                            admin.auth_level = ?user.auth,
                            admin.has_permission = false,
                            role_name = L::ROLE_NAME,
                            uri = %req.uri()
                        );
                        break 'auth Err(AuthorizeError::AuthorizeLevel(
                            AuthorizationAccessDenyError::new::<L>(),
                        ));
                    };

                    info!(
                        admin.name = user.username,
                        admin.auth_level = ?user.auth,
                        permission.accept = verify
                    );

                    Ok(req.tap_mut(|req| {
                        req.extensions_mut()
                            .insert(AuthorizedUser(user))
                            .expect_none_or_log("Authorize Layer Exist")
                    }))
                }
                .map_err(|err| RespResult::<(), _>::Err(err).into_response());

                result
            }
            .instrument(tracing::info_span!("authorization")),
        )
    }
}
