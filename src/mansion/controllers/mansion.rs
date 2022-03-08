use crate::{
    mansion::{db_ops::load_mansion::LoadMansion, modules::mansion::checkers::mansion::IdChecker},
    utils::{
        data_checker::PretreatChecker,
        req_pretreatment::{
            db_operate::DbOp,
            prefabs::{Null, PathValue, ToRResult},
        },
    },
};
use actix_web::get;
use rresult::{RResult, Wrap};

use crate::{
    mansion::{error::MansionError, modules::mansion::Mansion},
    utils::req_pretreatment::ReqPretreatment,
};

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
