use crate::{
    mansion::{db_ops::load_mansion::LoadMansion, pretreatments::split_id::SplitId},
    utils::req_pretreatment::{db_operate::DbOp, prefabs::ToRResult},
};
use actix_web::get;
use rresult::{RResult, Wrap};

use crate::{
    mansion::{error::MansionError, modules::mansion::Mansion},
    utils::req_pretreatment::ReqPretreatment,
};

#[get("/{id}")]
pub(super) async fn get_mansion(
    mansion: ReqPretreatment<ToRResult<DbOp<LoadMansion, SplitId>>>,
) -> RResult<Wrap<Mansion>, MansionError> {
    let m = mansion.unwrap()?;
    RResult::wrap_ok(m)
}
