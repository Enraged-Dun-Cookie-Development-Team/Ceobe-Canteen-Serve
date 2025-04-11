use http::request::Parts;
use tracing::info;
use persistence::ceobe_user::models::models::UserMobId;
use persistence::ceobe_user::{ToCeobe, ToCeobeUser};
use persistence::help_crates::StatusErr;
use crate::{AuthorizedUser, AuthorizeLayer, AuthorVerifier};
use persistence::help_crates::{ErrPrefix,HttpCode};
use persistence::help_crates::bool_or::TrueOrError;
use persistence::help_crates::futures::future::BoxFuture;
use persistence::mongodb::MongoDatabaseOperate;
use persistence::operate::FromRequestParts;
use crate::mob_user::configure::get_authorize_information;

pub type MobUserInfo = UserMobId;

pub type MobUserAuthorizeLayer = AuthorizeLayer<MobUser>;

pub type AuthorizedMobUser = AuthorizedUser<MobUserInfo>;

#[derive(Clone,Default)]
pub struct MobUser;

impl AuthorVerifier for MobUser {
    type AuthorizedUser = MobUserInfo;
    type Error = MobUserAuthorizeError;
    type Future = BoxFuture<'static,Result<MobUserInfo,Self::Error>>;

    fn authorize(&mut self, mut request_parts: Parts) -> Self::Future {
        Box::pin(
            async move{
        let mob_id = get_authorize_information(&request_parts)
            .ok_or(MobUserAuthorizeError::MobIdFieldNotFound)?.to_string();
                
                let db = MongoDatabaseOperate::from_request_parts(&mut request_parts,&()).await.unwrap();
                
                db.ceobe().user().property().is_exist_user(&mob_id).await
                    .map_err(|_|MobUserAuthorizeError::UserDatabaseOperateError)?
                .true_or_with(||MobUserAuthorizeError::MobIdNotExist(mob_id.clone()))?;

                info!(user.mob_id = %mob_id);
                
                Ok(UserMobId{mob_id:mob_id.to_string()})
            }
        )
        
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
