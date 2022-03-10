use crate::serves::mansion::error::MansionError;
use crate::serves::mansion::modules::mansion::Mansion;
use crate::serves::mansion::modules::mansion::checkers::mansion::IdChecker;
use crate::{
    utils::{
        data_checker::PretreatChecker,
        req_pretreatment::{
            db_operate::DbOp,
            prefabs::{Null, PathValue, ToRResult}, ReqPretreatment,
        },
    }, serves::mansion::db_ops::load_mansion::LoadMansion,
};
use actix_web::get;
use rresult::{RResult, Wrap};


type LoadingTargetMansionFromDb = ReqPretreatment<
    ToRResult<DbOp<LoadMansion, PretreatChecker<Null, PathValue<String>, IdChecker>>>,
>;

#[get("/{id}")]
pub(super) async fn get_mansion(
    mansion: LoadingTargetMansionFromDb,
) -> RResult<Wrap<Mansion>, MansionError> {
    let m = mansion.unwrap()?;
    RResult::wrap_ok(m)
}
