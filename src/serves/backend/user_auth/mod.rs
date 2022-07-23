use axum_prehandle::{prefabs::json::JsonPayload, PreRespHandling};
use resp_result::RespResult;

use self::{
    checker::user::{UsernameChecker, UsernameUncheck},
    error::AdminUserError,
};
use crate::utils::data_checker::PreLiteChecker;

mod checker;
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
