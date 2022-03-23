use crate::serves::mansion::error::MansionError;
use crate::serves::mansion::modules;
use crate::serves::mansion::modules::mansion::checkers::mansion::IdChecker;
use crate::serves::mansion::modules::mansion::Mansion;
use crate::utils::mongodb_utils::mongo_manager::MongoManager;
use crate::utils::req_pretreatment::prefabs::{Json, MapErr};
use crate::{
    serves::mansion::db_ops::load_mansion::LoadMansion,
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
use rresult::{RResult, Wrap, IntoRResultWithCodeError};

type LoadingTargetMansionFromDb = ReqPretreatment<
    ToRResult<DbOp<LoadMansion, PretreatChecker<Null, PathValue<String>, IdChecker>>>,
>;

crate::quick_struct! {
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
pub(super) async fn save_mansion(
    web::Query(SaveMansionId { id }): web::Query<SaveMansionId>,
) -> RResult<Wrap<()>, MansionError> {
    log::info!("saving id {:?}", id);
    unimplemented!()
}

#[post("/upload")]
pub(super) async fn save_mongo_mansion(
    body: ReqPretreatment<ToRResult<MapErr<Json<Mansion>,MansionError>>>,
    db: web::Data<MongoManager>,
) -> RResult<Wrap<()>, MansionError> {
    let db = db.get_db("mansion").unwrap().collection::<Mansion>().unwrap();

    let body=body.unwrap()?;

    db.insert_one(body, None).await.unwrap();
    RResult::wrap_ok(())
}
