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
use request_pretreat::{
    prefabs::{JsonPayload, ToRespResult},
    Pretreatment,
};
type AdminUserRResult<T> = RespResult<T, error::AdminUserError>;

type UsernamePretreatment = Pretreatment<
    ToRespResult<
        PreLiteChecker<
            JsonPayload<UsernameUncheck>,
            UsernameChecker,
            AdminUserError,
        >,
    >,
>;
