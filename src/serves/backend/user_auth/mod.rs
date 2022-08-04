use axum_prehandle::{prefabs::json::JsonPayload, PreRespHandling};
use orm_migrate::sql_models::admin_user::checkers::username::{
    UsernameChecker, UsernameUncheck,
};
use resp_result::RespResult;

use self::error::AdminUserError;
use crate::utils::data_checker::PreLiteChecker;

mod controllers;
mod error;
mod view;

type AdminUserRResult<T> = RespResult<T, error::AdminUserError>;

type UsernamePretreatment = PreRespHandling<
    PreLiteChecker<
        JsonPayload<UsernameUncheck>,
        UsernameChecker,
        AdminUserError,
    >,
>;
