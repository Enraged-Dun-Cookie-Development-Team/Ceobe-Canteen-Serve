use std::marker::PhantomData;

use axum::extract::FromRequestParts;
use crypto_str::inner_encoders::bcrypt::BcryptError;
use http::request::Parts;
use persistence::{
    admin::{
        ToAdmin,
        user::{OperateError, ToUser},
    },
    help_crates::{StatusErr, futures::future::BoxFuture},
    mysql::SqlDatabaseOperate,
};
use status_err::generated_error::unauthorized_kind::{
    AdminTokenInfoNotFoundError, AdminTokenInvalidError,
    AdminTokenNotFoundError,
};
use tracing::{info, warn};

use crate::{
    AuthorVerifier, AuthorizeLayer, AuthorizedUser,
    admin::{
        configure::get_authorize_information,
        roles::{AuthorizationAccessDenyError, UserRoleVerify},
        token_payload::UserClaim,
    },
    token_conv::JwtTokenConv,
};

pub type AdminUser = persistence::admin::models::Model;

pub type AdminAuthorizeLayer<L> = AuthorizeLayer<Admin<L>>;
pub type AuthorizedAdminUser = AuthorizedUser<AdminUser>;

pub struct Admin<L: UserRoleVerify>(PhantomData<L>);

impl<L: UserRoleVerify> Default for Admin<L> {
    fn default() -> Self { Admin(PhantomData) }
}

impl<L: UserRoleVerify> Clone for Admin<L> {
    fn clone(&self) -> Self { Admin(PhantomData) }
}

impl<L> AuthorVerifier for Admin<L>
where
    L: UserRoleVerify + 'static,
{
    type AuthorizedUser = AdminUser;
    type Error = AdminAuthorizeError;
    type Future =
        BoxFuture<'static, Result<Self::AuthorizedUser, Self::Error>>;

    fn authorize(&mut self, request_parts: Parts) -> Self::Future {
        Box::pin(admin_authorize::<L>(request_parts))
    }
}

async fn admin_authorize<L: UserRoleVerify>(
    mut request_part: Parts,
) -> Result<AdminUser, AdminAuthorizeError> {
    let UserClaim {
        id,
        password_version,
        ..
    } = get_authorize_information(&request_part)
        .ok_or(AdminAuthorizeError::TokenInfoNotFound)
        .and_then(|token| {
            UserClaim::from_jwt_token(&token).map_err(Into::into)
        })?;

    let db = SqlDatabaseOperate::from_request_parts(&mut request_part, &())
        .await
        .unwrap();

    let user = db
        .admin()
        .user()
        .find_user_with_version_verify(
            id,
            password_version,
            |user| user,
            || AdminAuthorizeError::TokenInvalid,
        )
        .await
        .map_err(|err| {
            match err {
                OperateError::UserNotExist => {
                    AdminAuthorizeError::TokenInfoNotFound
                }
                e => AdminAuthorizeError::from(e),
            }
        })??;

    let verify @ true = L::access_verify(&user.auth)
    else {
        warn!(
            admin.name = user.username,
            admin.auth_level = ?user.auth,
            admin.has_permission = false,
            role_name = L::ROLE_NAME,
            uri = %request_part.uri
        );
        return Err(AdminAuthorizeError::AuthorizeLevel(
            AuthorizationAccessDenyError::new::<L>(),
        ));
    };

    info!(
        admin.name = user.username,
        admin.auth_level = ?user.auth,
        permission.accept = verify
    );
    Ok(user)
}

#[derive(Debug, thiserror::Error, StatusErr)]
#[status_err(resp_err)]
pub enum AdminAuthorizeError {
    #[error("JWT 解析异常 {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("密码加密、解析异常")]
    Bcrypt(#[from] BcryptError),
    #[error(transparent)]
    UserDbOperate(#[from] OperateError),
    #[error(transparent)]
    AuthorizeLevel(#[from] AuthorizationAccessDenyError),

    #[error("Token 信息未找到")]
    #[status_err(err(bind = "AdminTokenNotFoundError"))]
    TokenNotFound,

    #[error("Token 已经失效")]
    #[status_err(err(bind = "AdminTokenInvalidError"))]
    TokenInvalid,

    #[error("Token 对应信息不存在")]
    #[status_err(err(bind = "AdminTokenInfoNotFoundError"))]
    TokenInfoNotFound,
}
