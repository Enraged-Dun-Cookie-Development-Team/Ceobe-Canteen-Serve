use crate::{
    serves::mansion::{
        error::{MansionError, MansionNotFound},
        modules::{
            mansion::{
                MIdCheckerPretreat, Mansion, MansionCheckerPretreat, MansionId, Mid,
                OptionMidCheckerPretreat, ViewMansion,
            },
            MansionDb,
        },
        MansionRResult,
    },
    utils::{
        mongodb_utils::db_selector::MongoDbSelector,
        req_pretreatment::{
            prefabs::{MapErr, ToRResult},
            ReqPretreatment,
        },
    },
};
use actix_web::{get, post};
use futures::StreamExt;
use mongodb::{bson::doc, options::FindOptions};
use resp_result::RespResult;

#[post("/upload")]
pub(super) async fn save_mansion(
    ReqPretreatment(mid): ReqPretreatment<
        ToRResult<MapErr<OptionMidCheckerPretreat, MansionError>>,
    >,
    ReqPretreatment(json): ReqPretreatment<ToRResult<MapErr<MansionCheckerPretreat, MansionError>>>,
    ReqPretreatment(db): ReqPretreatment<
        ToRResult<MapErr<MongoDbSelector<MansionDb>, MansionError>>,
    >,
) -> MansionRResult<()> {
    let mid = mid?.id;
    let data = json?;
    let db = db?;
    println!("{:#?}", data);
    match mid {
        Some(MansionId { main_id, minor_id }) => {
            let filter = doc! {
                "id" : {
                    "main_id":main_id,
                    "minor_id":minor_id as i32
                }
            };

            let resp = db
                .doing(|collect| async move {
                    let task = collect.find_one_and_replace(filter, data, None);
                    task.await?;
                    Ok(())
                })
                .await;
            let _ = MansionRResult::from(resp)?;
        }
        None => {
            let resp = db
                .doing(|c| async move {
                    c.insert_one(data, None).await?;
                    Ok(())
                })
                .await;
            let _ = MansionRResult::from(resp)?;
        }
    }
    Ok(()).into()
}
#[get("/getInfo")]
pub(super) async fn get_mansion(
    ReqPretreatment(mid): ReqPretreatment<ToRResult<MapErr<MIdCheckerPretreat, MansionError>>>,
    ReqPretreatment(db): ReqPretreatment<
        ToRResult<MapErr<MongoDbSelector<MansionDb>, MansionError>>,
    >,
) -> MansionRResult<ViewMansion> {
    let MansionId { main_id, minor_id } = mid?.id;

    let filter = doc! {
        "id" : {
            "main_id":main_id
            ,
            "minor_id":minor_id as i32
        }
    };
    let resp = db?
        .doing(|collect| async move { collect.find_one(filter, None).await })
        .await;

    let data = MansionRResult::from(resp)?.ok_or(MansionNotFound.into());
    let data: Mansion = MansionRResult::from(data)?;

    MansionRResult::ok(data.into())
}
#[get("/getId")]
pub(super) async fn get_all_id(
    ReqPretreatment(db): ReqPretreatment<
        ToRResult<MapErr<MongoDbSelector<MansionDb>, MansionError>>,
    >,
) -> MansionRResult<Vec<String>> {
    let resp = db?
        .doing::<_, Mansion, _, MansionError, _>(|collect| async move {
            let collect = collect.clone_with_type::<Mid>();
            let mut vec = collect
                .find(
                    None,
                    FindOptions::builder().projection(doc! {"id":1i32}).build(),
                )
                .await?;
            let mut resp = Vec::new();
            while let Some(v) = vec.next().await {
                let v = v?;
                resp.push(v);
            }
            Ok(resp)
        })
        .await;

    let resp = RespResult::from(resp)?
        .into_iter()
        .map(|id| id.id.to_string())
        .collect();

    Ok(resp).into()
}
