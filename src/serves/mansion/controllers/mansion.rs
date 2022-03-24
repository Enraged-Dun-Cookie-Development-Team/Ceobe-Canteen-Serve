use crate::{
    serves::mansion::{
        db_ops::load_mansion::LoadMansion,
        modules::mansion::{checkers::mansion::IdChecker, Mansion},
        MansionRResult,
    },
    utils::{
        data_checker::PretreatChecker,
        req_pretreatment::{
            db_operate::DbOp,
            prefabs::{Null, PathValue, ToRResult},
            ReqPretreatment,
        },
    },
};
use actix_web::{get, post, web};

type LoadingTargetMansionFromDb = ReqPretreatment<
    ToRResult<DbOp<LoadMansion, PretreatChecker<Null, PathValue<String>, IdChecker>>>,
>;

crate::quick_struct! {
    pub SaveMansionId{
        id:Option<String>
    }
}

#[get("/{id}")]
pub(super) async fn get_mansion(mansion: LoadingTargetMansionFromDb) -> MansionRResult<Mansion> {
    mansion.unwrap()
}

#[post("/")]
pub(super) async fn save_mansion(
    web::Query(SaveMansionId { id }): web::Query<SaveMansionId>,
) -> MansionRResult<()> {
    log::info!("saving id {:?}", id);
    unimplemented!()
}
