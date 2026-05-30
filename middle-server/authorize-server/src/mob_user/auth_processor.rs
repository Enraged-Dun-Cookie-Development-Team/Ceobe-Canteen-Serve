use http::request::Parts;
use persistence::{
    ceobe_user::{ToCeobe, ToCeobeUser, models::models::UserMobId},
    help_crates::{
        StatusErr, bool_or::TrueOrError, futures::future::BoxFuture,
    },
    mongodb::MongoDatabaseOperate,
    operate::FromRequestParts,
};
use status_err::generated_error::unauthorized_kind::{
    MobIdFieldNotFoundError, MobIdNotExistError, NoMobIdLayerError,
    UserDatabaseOperateError,
};
use tracing::info;

use crate::{
    AuthorVerifier, AuthorizeLayer, AuthorizedUser,
    mob_user::configure::get_authorize_information,
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
    #[status_err(err(bind = "MobIdFieldNotFoundError"))]
    MobIdFieldNotFound,

    #[error("Mob id:{0} 不存在")]
    #[status_err(err(bind = "MobIdNotExistError"))]
    MobIdNotExist(String),

    #[error("缺少MobId鉴权中间件")]
    #[status_err(err(bind = "NoMobIdLayerError"))]
    NoMobIdLayer,

    #[error("Mongo用户表查询失败")]
    #[status_err(err(bind = "UserDatabaseOperateError"))]
    UserDatabaseOperateError,
}
