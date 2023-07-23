use std::marker::PhantomData;

use async_trait::async_trait;
use axum_prehandle::PreHandler;
use time_usage::{async_time_usage_with_name, sync_time_usage_with_name};

use crate::utils::user_authorize::{
    AuthInfo, error::AuthError, TokenAuth, VerifiedAuthInfo,
};

use super::{AuthLevelVerify, error::UnacceptableAuthorizationLevelError};

pub struct AuthLevel<L: AuthLevelVerify>(PhantomData<L>);

#[async_trait]
impl<B: Send, L: AuthLevelVerify> PreHandler<B> for AuthLevel<L> {
    type Output = VerifiedAuthInfo;
    type Rejection = AuthError;

    async fn handling(
        request: &mut axum::extract::RequestParts<B>,
    ) -> Result<Self::Output, Self::Rejection> {
        let info = async_time_usage_with_name(
            "解析用户Token",
            TokenAuth::handling(request),
        )
        .await?;
        let AuthInfo {
            id,
            auth,
            username,
            password: pwd,
            ..
        } = info;
        sync_time_usage_with_name(
            "校验用户权限等级是否匹配",
            || {
                if L::verify(&auth) {
                    Ok(VerifiedAuthInfo { id, username, pwd })
                }
                else {
                    Err(UnacceptableAuthorizationLevelError::new(
                        L::auth_name(),
                    )
                    .into())
                }
            },
        )
    }
}
