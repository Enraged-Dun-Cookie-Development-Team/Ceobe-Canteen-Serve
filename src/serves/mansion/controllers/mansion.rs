use crate::{
    serves::mansion::{
        error::MansionError,
        modules::mansion::{MansionCheckerPretreat, ViewMansion},
        MansionRResult,
    },
    utils::req_pretreatment::{
        prefabs::{MapErr, ToRResult},
        ReqPretreatment,
    },
};
use actix_web::{get, post, web};

crate::quick_struct! {
    pub MansionId{
        #[serde(alias="idBefore",alias="mansionId")]
        id:Option<String>
    }
}

#[post("/upload")]
pub(super) async fn save_mansion(
    web::Query(MansionId { id:_ }): web::Query<MansionId>,
    ReqPretreatment(json): ReqPretreatment<ToRResult<MapErr<MansionCheckerPretreat, MansionError>>>,
) -> MansionRResult<()> {
    let data = json?;
    
    println!("{:#?}", data);
    Ok(()).into()
}
#[get("/getInfo")]
pub(super) async fn get_mansion(
    web::Query(MansionId { id:_ }): web::Query<MansionId>,
) -> MansionRResult<ViewMansion> {
    todo!("Get the mongodb and read data")
}
