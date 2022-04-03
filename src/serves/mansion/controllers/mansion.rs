use actix_web::{get, post};
use chrono::Local;
use futures::StreamExt;
use mongodb::{bson::doc, options::FindOptions};

use super::{
    super::MansionRResult, MansionBodyCheckerPretreatment,
    MansionMongoDbPretreatment, OptionMidCheckerPretreatment,
};
use crate::{
    serves::mansion::{
        controllers::{MansionAuthentication, MidCheckerPretreatment},
        error::{MansionIdExist, MansionNotFound},
        modules::mansion::{MansionId, Mid, ModelMansion, ModifyAt},
        view::ViewMansion,
    },
    utils::req_pretreatment::ReqPretreatment,
};

#[post("/upload")]
pub(super) async fn save_mansion(
    _: MansionAuthentication,
    ReqPretreatment(mid): OptionMidCheckerPretreatment,
    ReqPretreatment(json): MansionBodyCheckerPretreatment,
    ReqPretreatment(db): MansionMongoDbPretreatment,
) -> MansionRResult<()> {
    let mid = mid.id;
    let data = json;
    let db = db;

    match mid {
        Some(MansionId { main_id, minor_id }) => {
            let filter = doc! {
                "id" : {
                    "main_id":main_id,
                    "minor_id":minor_id as i32
                }
            };

            // loading old data modify info
            let o_filter = filter.clone();
            let old_info = db
                .doing::<_, ModelMansion, _, _>(|collect| {
                    async move {
                        let collect = collect.clone_with_type::<ModifyAt>();
                        collect.find_one(o_filter, None).await
                    }
                })
                .await?
                .unwrap_or_default();

            // update database
            db.doing(|collect| {
                async move {
                    let task = collect.find_one_and_replace(
                        filter,
                        ModelMansion::with_modify_time(
                            data,
                            old_info.now_modify(),
                        ),
                        None,
                    );
                    task.await?;
                    Ok(())
                }
            })
            .await?;
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
                .doing::<_, ModelMansion, _, _>(|collection| async move {
                    collection.count_documents(filter, None).await
                })
                .await?;

            if check == 0 {
                db.doing::<_, ModelMansion, _, _>(|c| {
                    async move {
                        c.insert_one(ModelMansion::from(data), None).await?;
                        Ok(())
                    }
                })
                .await?;
            }
            else {
                MansionRResult::<()>::err(MansionIdExist.into())?;
            }
        }
    }
    Ok(()).into()
}

#[get("/getInfo")]
pub(super) async fn get_mansion(
    _: MansionAuthentication,
    ReqPretreatment(mid): MidCheckerPretreatment,
    ReqPretreatment(db): MansionMongoDbPretreatment,
) -> MansionRResult<ViewMansion> {
    let MansionId { main_id, minor_id } = mid.id;
    let db = db;

    let filter = doc! {
        "id" : {
            "main_id":main_id
            ,
            "minor_id":minor_id as i32
        }
    };
    let data = db
        .doing::<_, ModelMansion, _, _>(|collect| {
            async move { collect.find_one(filter, None).await }
        })
        .await?
        .ok_or(MansionNotFound)?;

    MansionRResult::ok(data.into())
}

#[get("/getId")]
pub(super) async fn get_all_id(
    _: MansionAuthentication,
    ReqPretreatment(db): MansionMongoDbPretreatment,
) -> MansionRResult<Vec<String>> {
    // 最近60天
    let now = Local::now().naive_local() - (chrono::Duration::days(60));
    let now = mongodb::bson::DateTime::from_millis(now.timestamp_millis());
    log::info!("mansion after {:?}", now);
    let filter = doc! {
        "create_time":{
            "$gte":now
        }
    };

    let resp = db
        .doing::<_, ModelMansion, _, _>(|collect| {
            async move {
                let collect = collect.clone_with_type::<Mid>();
                let mut vec = collect
                    .find(
                        filter,
                        FindOptions::builder()
                            .projection(doc! {"id":1i32})
                            .build(),
                    )
                    .await?;
                let mut resp = Vec::new();
                while let Some(v) = vec.next().await {
                    let v = v?;
                    resp.push(v);
                }
                Ok(resp)
            }
        })
        .await?
        .into_iter()
        .map(|id| id.id.to_string())
        .collect();

    Ok(resp).into()
}
#[post("/delete")]
pub(super) async fn remove_mansion(
    _: MansionAuthentication,
    ReqPretreatment(db): MansionMongoDbPretreatment,
    ReqPretreatment(mid): MidCheckerPretreatment,
) -> MansionRResult<()> {
    let MansionId { main_id, minor_id } = mid.id;
    let filter = doc! {
        "id" : {
            "main_id":main_id
            ,
            "minor_id":minor_id as i32
        }
    };
    db.doing::<_, ModelMansion, _, ()>(|collect| {
        async move {
            collect.delete_one(filter, None).await?;
            Ok(())
        }
    })
    .await?;

    Ok(()).into()
}
