use crate::{
    serves::mansion::{
        error::MansionError,
        modules::mansion::{
            MIdCheckerPretreat, MansionCheckerPretreat, OptionMidCheckerPretreat, ViewMansion,
        },
        MansionRResult,
    },
    utils::req_pretreatment::{
        prefabs::{MapErr, ToRResult},
        ReqPretreatment,
    },
};
use actix_web::{get, post};

crate::quick_struct! {
    pub MansionId{
        #[serde(alias="idBefore",alias="mansionId")]
        id:Option<String>
    }
}

#[post("/upload")]
pub(super) async fn save_mansion(
    ReqPretreatment(mid): ReqPretreatment<
        ToRResult<MapErr<OptionMidCheckerPretreat, MansionError>>,
    >,
    ReqPretreatment(json): ReqPretreatment<ToRResult<MapErr<MansionCheckerPretreat, MansionError>>>,
) -> MansionRResult<()> {
    let _mid = mid?.id;
    let data = json?;
    println!("{:#?}", data);
    Ok(()).into()
}
#[get("/getInfo")]
pub(super) async fn get_mansion(
    ReqPretreatment(mid): ReqPretreatment<ToRResult<MapErr<MIdCheckerPretreat, MansionError>>>,
) -> MansionRResult<ViewMansion> {
    let _mid = mid?;
    todo!("Get the mongodb and read data")
}
