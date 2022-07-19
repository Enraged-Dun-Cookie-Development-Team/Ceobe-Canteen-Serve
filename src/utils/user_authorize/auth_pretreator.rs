use futures::Future;
use lazy_static::__Deref;
use sql_connection::get_sql_database;
use time_usage::{async_time_usage_with_name, sync_time_usage_with_name};

use super::{
    config::TokenHeader as Token, error::AuthError,
    valid_token::decrpyt_token, AuthInfo,
};
use crate::{
    utils::{
        data_struct::header_info::HeaderInfo,
        req_pretreatment::Pretreatment,
        user_authorize::error::{TokenInvalid, TokenNotFound, UserNotFound},
    },
};
use crate::models::common::sql::sql_models::user;

pub struct TokenAuth;


impl Pretreatment for TokenAuth {
    // 异步返回的fut
    type Fut = impl Future<Output = Result<Self::Resp, Self::Err>>;
    // 返回类型
    type Resp = AuthInfo;

    // 异常
    type Err = AuthError;

    fn proc(
        req: &actix_web::HttpRequest, payload: &mut actix_http::Payload,
    ) -> Self::Fut {
        let db = get_sql_database();
        let token = HeaderInfo::<Token>::proc(req, payload);

        async move {
            // 获取token
            let token =
                async_time_usage_with_name("获取用户token信息", async {
                    let token = token.await?;
                    let token = token.get_one().ok_or(TokenNotFound)?;
                    decrpyt_token(token).map_err(AuthError::from)
                })
                .await?;

            use sea_orm::EntityTrait;

            // 获取用户信息
            let user_info = async_time_usage_with_name(
                "查询用户信息",
                user::Entity::find_by_id(token.id).one(db.deref().deref()),
            )
            .await?
            .ok_or(UserNotFound)?;
            let user::Model {
                id,
                password,
                auth,
                username,
                num_pwd_change,
            } = user_info;
            sync_time_usage_with_name("校验Token信息", || {
                if num_pwd_change == token.num_pwd_change {
                    Ok(AuthInfo {
                        id,
                        password,
                        auth,
                        username,
                        num_pwd_change,
                    })
                }
                else {
                    Err(TokenInvalid.into())
                }
            })
        }
    }
}
