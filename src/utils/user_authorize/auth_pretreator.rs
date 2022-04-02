use std::borrow::Cow;

use actix_web::web::Data;
use crypto_str::Encoder;
use futures::Future;
use lazy_static::__Deref;
use serde::{Deserialize, Serialize};

use super::{
    config::TokenHeader as Token, error::AuthError,
    valid_token::decrpyt_token, AuthInfo, PasswordEncoder,
};
use crate::{
    database::ServeDatabase,
    utils::{
        data_struct::header_info::HeaderInfo,
        req_pretreatment::Pretreatment,
        user_authorize::error::{PasswordWrong, TokenNotFound, UserNotFound},
    },
};

pub struct TokenAuth;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AuthLevel {
    #[serde(rename = "chef")]
    Chef,
    #[serde(rename = "cooker")]
    Cooker,
    #[serde(rename = "architect")]
    Architect,
}

impl From<db_entity::sea_orm_active_enums::Auth> for AuthLevel {
    fn from(auth: db_entity::sea_orm_active_enums::Auth) -> Self {
        match auth {
            db_entity::sea_orm_active_enums::Auth::Chef => Self::Chef,
            db_entity::sea_orm_active_enums::Auth::Cooker => Self::Cooker,
            db_entity::sea_orm_active_enums::Auth::Architect => {
                Self::Architect
            }
        }
    }
}

impl Pretreatment for TokenAuth {
    // 异常
    type Err = AuthError;
    // 返回类型
    type Resp = AuthInfo;

    // 异步返回的fut
    type Fut = impl Future<Output = Result<Self::Resp, Self::Err>>;

    fn call<'r>(
        req: &'r actix_web::HttpRequest, payload: &'r mut actix_http::Payload,
    ) -> Self::Fut {
        let db = req
            .app_data::<Data<ServeDatabase<sea_orm::DatabaseConnection>>>()
            .expect("Database Connect Not Found In AppData")
            .clone();
        let token = HeaderInfo::<Token>::call(req, payload);

        async move {
            // 获取token
            let token = token.await?;
            let token = token.get_one().ok_or(TokenNotFound)?;
            let token = decrpyt_token(token)?;

            use db_entity::user;
            use sea_orm::EntityTrait;

            // 获取用户信息
            let user_info = user::Entity::find_by_id(token.id)
                .one(db.deref().deref())
                .await?
                .ok_or(UserNotFound)?;
            let user::Model {
                id,
                password,
                auth,
                username,
            } = user_info;

            if PasswordEncoder::verify(
                &Cow::Owned(password.clone()),
                &token.password,
            )? {
                Ok(AuthInfo {
                    id,
                    password,
                    auth: auth.into(),
                    username,
                })
            }
            else {
                Err(PasswordWrong.into())
            }
        }
    }
}
