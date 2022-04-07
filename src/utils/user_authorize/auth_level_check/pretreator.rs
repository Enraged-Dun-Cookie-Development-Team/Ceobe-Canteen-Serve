use std::marker::PhantomData;

use futures::Future;
use time_usage::{async_time_usage_with_name, sync_time_usage_with_name};

use super::{error::UnacceptableAuthorizationLevelError, AuthLevelVerify};
use crate::utils::{
    req_pretreatment::Pretreatment,
    user_authorize::{
        error::AuthError, AuthInfo, TokenAuth, VerifiedAuthInfo,
    },
};

pub struct AuthLevel<L: AuthLevelVerify>(PhantomData<L>);

impl<L: AuthLevelVerify> Pretreatment for AuthLevel<L> {
    type Err = AuthError;
    type Resp = VerifiedAuthInfo;

    type Fut = impl Future<Output = Result<Self::Resp, Self::Err>>;

    fn proc(
        req: &actix_web::HttpRequest, payload: &mut actix_http::Payload,
    ) -> Self::Fut {
        let task = TokenAuth::proc(req, payload);

        async move {
            let info =
                async_time_usage_with_name("解析用户Token", task).await?;
            let AuthInfo {
                id,
                auth,
                username,
                password: pwd,
                num_change: _,
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
}
