use std::borrow::Cow;

use actix_web::web::Data;
use crypto_str::Encoder;
use futures::Future;
use http::StatusCode;
use lazy_static::__Deref;
use serde::{Deserialize, Serialize};
use status_err::ErrPrefix;

use super::{valid_token::decrpyt_token, PasswordEncoder, auth_level};
use crate::{
    database::ServeDatabase,
    error_generate, header_captures,
    utils::{
        data_struct::header_info::HeaderInfo, req_pretreatment::Pretreatment,
    },
};

header_captures!(pub Token:"token");

pub struct TokenAuth;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AuthLevel {
    Chef,
    Cooker,
    Architect,
}

crate::quick_struct! {
    /// 用户权限信息
    pub AuthInfo{
        id: i32
        /// 权限
        auth: AuthLevel
        username: String
    }

    pub VerifiedAuthInfo{
        id:i32
        username:String
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
            let token = token.await?;
            let token = token.get_one().ok_or(TokenNotFound)?;
            let token = decrpyt_token(token)?;

            use db_entity::{sea_orm_active_enums::Auth, user};
            use sea_orm::EntityTrait;

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
                &Cow::Owned(password),
                &token.password.as_str(),
            )? {
                Ok(AuthInfo {
                    id,
                    auth: match auth {
                        Auth::Chef => AuthLevel::Chef,
                        Auth::Cooker => AuthLevel::Cooker,
                        Auth::Architect => AuthLevel::Architect,
                    },
                    username,
                })
            }
            else {
                Err(PasswordWrong.into())
            }
        }
    }
}

status_err::status_error!(pub TokenNotFound [
                                            ErrPrefix::UNAUTHORIZED,
                                            0001
                                            ]=>"缺少Token字段");
status_err::status_error!(pub PasswordWrong [
                                            ErrPrefix::UNAUTHORIZED,
                                            0004
                                            ]=>"密码错误");
status_err::status_error!(pub UserNotFound [
                                            ErrPrefix::UNAUTHORIZED,
                                            0003:StatusCode::NOT_FOUND
                                            ]=>"Token对应信息不存在");
error_generate!(
    pub AuthError

    Jwt=jwt::Error
    NoToken = TokenNotFound
    NoUser = UserNotFound
    Password = PasswordWrong
    Actix = actix_web::Error
    Db = sea_orm::DbErr
    Bcrypto = bcrypt::BcryptError
    AuthLevel = auth_level::UnacceptableAuthorizationLevelError
    
);
