use axum::extract::rejection::QueryRejection;
use axum_resp_result::RespResult;
use bakery_logic::error::LogicError;
use checker::QueryCheckExtract;
use persistence::bakery::{
    mansion::OperateError,
    models::mansion::{
        checkers::CheckError, preludes::id_checker::MidChecker,
    },
};

use crate::error_generate;

error_generate!(
    pub MansionError
    // request entity error
    Query = QueryRejection
    //db error
    Mongo = OperateError
    Checker = CheckError
    ModifyVerify = modify_cache::Error
    BakeryLogic = LogicError
);

pub type MidCheckerPretreatment = QueryCheckExtract<MidChecker, MansionError>;

pub(super) type MansionRResult<T> = RespResult<T, MansionError>;
