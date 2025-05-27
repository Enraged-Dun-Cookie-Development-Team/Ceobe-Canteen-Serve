use http::request::Parts;
use persistence::{
    ceobe_user::{models::models::UserMobId, ToCeobe, ToCeobeUser},
    help_crates::{
        bool_or::TrueOrError, futures::future::BoxFuture, ErrPrefix,
        HttpCode, StatusErr,
    },
    mongodb::MongoDatabaseOperate,
    operate::FromRequestParts,
};
use tracing::info;

use crate::{
    mob_user::configure::get_authorize_information, AuthorVerifier,
    AuthorizeLayer, AuthorizedUser,
};

pub type MobUserInfo = UserMobId;

pub type MobUserAuthorizeLayer = AuthorizeLayer<MobUser>;

pub type AuthorizedMobUser = AuthorizedUser<MobUserInfo>;

#[derive(Clone, Default)]
pub struct MobUser;

impl AuthorVerifier for MobUser {
    type AuthorizedUser = MobUserInfo;
    type Error = MobUserAuthorizeError;
    type Future = BoxFuture<'static, Result<MobUserInfo, Self::Error>>;

    fn authorize(&mut self, mut request_parts: Parts) -> Self::Future {
        Box::pin(async move {
            let mob_id = get_authorize_information(&request_parts)
                .ok_or(MobUserAuthorizeError::MobIdFieldNotFound)?
                .to_string();

            let db = MongoDatabaseOperate::from_request_parts(
                &mut request_parts,
                &(),
            )
            .await
            .unwrap();

            db.ceobe()
                .user()
                .property()
                .is_exist_user(&mob_id)
                .await
                .map_err(|_| MobUserAuthorizeError::UserDatabaseOperateError)?
                .true_or_with(|| {
                    MobUserAuthorizeError::MobIdNotExist(mob_id.clone())
                })?;

            info!(user.mob_id = %mob_id);

            Ok(UserMobId {
                mob_id: mob_id.to_string(),
            })
        })
    }
}

#[derive(Debug, thiserror::Error, StatusErr)]
#[status_err(resp_err)]
pub enum MobUserAuthorizeError {
    #[error("Mob id 字段未找到")]
    #[status_err(err(
        prefix = "ErrPrefix::UNAUTHORIZED",
        err_code = 0x000B,
        resp_msg = "Mob字段不存在，请联系开发者"
    ))]
    MobIdFieldNotFound,

    #[error("Mob id:{0} 不存在")]
    #[status_err(err(
        prefix = "ErrPrefix::UNAUTHORIZED",
        err_code = 0x000C,
        resp_msg = "请携带正确的Mob Id进行请求"
    ))]
    MobIdNotExist(String),

    #[error("缺少MobId鉴权中间件")]
    #[status_err(err(
        prefix = "ErrPrefix::UNAUTHORIZED",
        err_code = 0x000D,
        http_code = "HttpCode::INTERNAL_SERVER_ERROR"
    ))]
    NoMobIdLayer,

    #[error("Mongo用户表查询失败")]
    #[status_err(err(
        prefix = "ErrPrefix::UNAUTHORIZED",
        err_code = 0x000E,
        resp_msg = "系统错误，请联系开发者"
    ))]
    UserDatabaseOperateError,
}
