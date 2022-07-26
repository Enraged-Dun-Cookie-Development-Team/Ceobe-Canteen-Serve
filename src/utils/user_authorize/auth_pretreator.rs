use async_trait::async_trait;
use axum_prehandle::PreHandler;
use orm_migrate::sql_models::admin_user::operate::{
    OperateError, UserSqlOperate,
};
use time_usage::async_time_usage_with_name;

use super::{
    config::TokenHeader as Token,
    error::{AuthError, TokenInfoNotFound},
    valid_token::decrypt_token,
    AuthInfo,
};
use crate::utils::{
    data_struct::header_info::HeaderInfo,
    user_authorize::error::{TokenInvalid, TokenNotFound},
};

pub struct TokenAuth;

#[async_trait]
impl<B: Send> PreHandler<B> for TokenAuth {
    type Output = AuthInfo;
    type Rejection = AuthError;

    async fn handling(
        request: &mut axum::extract::RequestParts<B>,
    ) -> Result<Self::Output, Self::Rejection> {
        let token = async_time_usage_with_name("获取用户token信息", async {
            let token = HeaderInfo::<Token>::handling(request)
                .await?
                .get_one()
                .ok_or(TokenNotFound)?;
            decrypt_token(token).map_err(AuthError::from)
        })
        .await?;

        let user_info = UserSqlOperate::find_user_with_version_verify(
            token.id as i64,
            token.num_pwd_change,
            |user| user,
            TokenInvalid,
        )
        .await
        .map_err(|err| {
            match err {
                OperateError::UserNotExist => {
                    AuthError::TokenInfoNotFound(TokenInfoNotFound)
                }
                err => AuthError::UserDbOperate(err),
            }
        })??;

        Ok(user_info)
    }
}
