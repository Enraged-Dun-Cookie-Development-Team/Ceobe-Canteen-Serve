use crate::{utils::req_pretreatment::ReqPretreatment, serves::mansion::{modules::mansion::{MansionId, Mansion, Mid}, error::{MansionError, MansionIdExist, MansionNotFound}, controllers::MidCheckerPretreatment, view::ViewMansion}};
use actix_web::{get, post};
use futures::StreamExt;
use mongodb::{bson::doc, options::FindOptions};
use resp_result::RespResult;

use super::{
    super::MansionRResult, MansionBodyCheckerPretreatment, MansionMongoDbPretreatment,
    OptionMidCheckerPretreatment,
};

#[post("/upload")]
pub(super) async fn save_mansion(
    ReqPretreatment(mid): OptionMidCheckerPretreatment,
    ReqPretreatment(json): MansionBodyCheckerPretreatment,
    ReqPretreatment(db): MansionMongoDbPretreatment,
) -> MansionRResult<()> {
    let mid = mid?.id;
    let data = json?;
    let db = db?;
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
            let MansionId { main_id, minor_id } = data.id.clone();
            let filter = doc! {
                "id" : {
                    "main_id":main_id,
                    "minor_id":minor_id as i32
                }
            };
            let check = db
                .doing::<_, Mansion, _, MansionError, _>(|collection| async move {
                    collection.count_documents(filter, None).await
                })
                .await;

            let check = RespResult::from(check)?;
            if check == 0 {
                let resp = db
                    .doing(|c| async move {
                        c.insert_one(data, None).await?;
                        Ok(())
                    })
                    .await;
                let _ = MansionRResult::from(resp)?;
            } else {
                MansionRResult::<()>::err(MansionIdExist.into())?;
            }
        }
    }
    Ok(()).into()
}

#[get("/getInfo")]
pub(super) async fn get_mansion(
    ReqPretreatment(mid): MidCheckerPretreatment,
    ReqPretreatment(db): MansionMongoDbPretreatment,
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
    ReqPretreatment(db): MansionMongoDbPretreatment,
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
#[post("/delete")]
pub(super) async fn remove_mansion(
    ReqPretreatment(db): MansionMongoDbPretreatment,
    ReqPretreatment(mid): MidCheckerPretreatment,
) -> MansionRResult<()> {
    let MansionId { main_id, minor_id } = mid?.id;
    let filter = doc! {
        "id" : {
            "main_id":main_id
            ,
            "minor_id":minor_id as i32
        }
    };

    let resp = db?
        .doing::<_, Mansion, _, MansionError, ()>(|collect| async move {
            collect.delete_one(filter, None).await?;
            Ok(())
        })
        .await;

    let _ = RespResult::from(resp)?;

    Ok(()).into()
}
