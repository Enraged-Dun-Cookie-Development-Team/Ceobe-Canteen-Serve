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
use actix_web::{get, post, web};
use rresult::{RResult, Wrap};


type LoadingTargetMansionFromDb = ReqPretreatment<
    ToRResult<DbOp<LoadMansion, PretreatChecker<Null, PathValue<String>, IdChecker>>>,
>;

crate::quick_struct!{
    pub SaveMansionId{
        id:Option<String>
    }
}

#[get("/{id}")]
pub(super) async fn get_mansion(
    mansion: LoadingTargetMansionFromDb,
) -> RResult<Wrap<Mansion>, MansionError> {
    let m = mansion.unwrap()?;
    RResult::wrap_ok(m)
}




#[post("/")]
pub(super) async fn save_mansion(web::Query(SaveMansionId{id}):web::Query<SaveMansionId>)->RResult<Wrap<()>,MansionError>{
    log::info!("saving id {:?}",id);
    unimplemented!()
}