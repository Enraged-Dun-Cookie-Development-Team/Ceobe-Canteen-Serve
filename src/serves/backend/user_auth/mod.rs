use axum_prehandle::{prefabs::json::JsonPayload, PreRespHandling};
use orm_migrate::sql_models::admin_user::checkers::username::{
    UsernameChecker, UsernameUncheck,
};
use page_size::request::{PageSizeChecker, PageSizeUncheck};
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

type PageSizePretreatment = PreRespHandling<
    PreLiteChecker<
        JsonPayload<PageSizeUncheck>,
        PageSizeChecker,
        AdminUserError,
    >,
>;
