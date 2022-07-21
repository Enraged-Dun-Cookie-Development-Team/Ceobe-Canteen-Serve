use futures::Future;
use orm_migrate::sql_models::user::{
    operate::CommonSqlOperate, CommonError,
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
    req_pretreatment::Pretreatment,
    user_authorize::error::{TokenInvalid, TokenNotFound},
};

pub struct TokenAuth;

impl Pretreatment for TokenAuth {
    // 异常
    type Err = AuthError;
    // 返回类型
    type Resp = AuthInfo;

    // 异步返回的fut
    type Fut = impl Future<Output = Result<Self::Resp, Self::Err>>;

    fn proc(
        req: &actix_web::HttpRequest, payload: &mut actix_http::Payload,
    ) -> Self::Fut {
        let token = HeaderInfo::<Token>::proc(req, payload).into_inner();

        async move {
            // 获取token
            let token =
                async_time_usage_with_name("获取用户token信息", async {
                    let token = token?;
                    let token = token.get_one().ok_or(TokenNotFound)?;
                    decrypt_token(token).map_err(AuthError::from)
                })
                .await?;

            let user_info = CommonSqlOperate::find_user_with_version_verify(
                token.id as i64,
                token.num_pwd_change,
                |user| user,
                TokenInvalid,
            )
            .await
            .map_err(|err| {
                match err {
                    CommonError::UserNotExist => {
                        AuthError::TokenInfoNotFound(TokenInfoNotFound)
                    }
                    err => AuthError::UserDbOperate(err),
                }
            })??;

            Ok(user_info)
        }
    }
}
