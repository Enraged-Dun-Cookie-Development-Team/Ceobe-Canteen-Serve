use std::fmt::Debug;

use ceobe_user::user::OperateError;
use http::StatusCode;
use status_err::{ErrPrefix, StatusErr};



#[derive(Debug, thiserror::Error, StatusErr)]
#[status_err(resp_err)]
pub enum MobVerifyError {   
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
        http_code = "StatusCode::INTERNAL_SERVER_ERROR"
    ))]
    NoMobIdLayer
}