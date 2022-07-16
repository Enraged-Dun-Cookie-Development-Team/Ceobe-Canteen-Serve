use chrono::Local;
use futures::StreamExt;
use mongodb::{bson::doc, options::FindOptions};
use time_usage::async_time_usage_with_name;

use super::{
    MansionRResult, MansionBodyCheckerPretreatment,
    MansionMongoDbPretreatment, OptionMidCheckerPretreatment, MansionAuthentication, MidCheckerPretreatment,
};
use crate::{
    models::mansion::preludes::*,
    router::BakeryMansionBackend,
    serves::backend::bakery_mansion::{
        error::{MansionIdExist, MansionNotFound},
        view::ViewMansion,
    },
    utils::req_pretreatment::ReqPretreatment,
};

impl BakeryMansionBackend {
    pub async fn save_mansion(
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
                log::info!("MansionId已提供 => 更新模式");
                let filter = doc! {
                    "id" : {
                        "main_id":main_id,
                        "minor_id":minor_id as i32
                    }
                };

                // loading old data modify info
                let o_filter = filter.clone();
                let old_info = async_time_usage_with_name(
                    "加载原有Mansion数据的更新信息",
                    db.doing::<_, ModelMansion, _, _>(|collect| {
                        async move {
                            let collect =
                                collect.clone_with_type::<ModifyAt>();
                            collect.find_one(o_filter, None).await
                        }
                    }),
                )
                .await?
                .unwrap_or_default();

                // update database
                async_time_usage_with_name(
                    "将新的Mansion替换原有Mansion",
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
                    }),
                )
                .await?;
            }
            None => {
                log::info!("MansionId未提供 => 新建模式");
                let MansionId { main_id, minor_id } = data.id.clone();
                let filter = doc! {
                    "id" : {
                        "main_id":main_id,
                        "minor_id":minor_id as i32
                    }
                };

                let check = async_time_usage_with_name(
                    "检查Mansion Id 是否已经被使用",
                    db.doing::<_, ModelMansion, _, _>(|collection| {
                        async move {
                            collection.count_documents(filter, None).await
                        }
                    }),
                )
                .await?
                    == 0;

                if check {
                    log::info!("MansionID <{}> 未被使用", data.id);
                    async_time_usage_with_name(
                        "新建Mansion",
                        db.doing::<_, ModelMansion, _, _>(|c| {
                            async move {
                                c.insert_one(ModelMansion::from(data), None)
                                    .await?;
                                Ok(())
                            }
                        }),
                    )
                    .await?;
                }
                else {
                    log::error!("MansionID <{}> 已经被使用", data.id);
                    MansionRResult::<()>::err(MansionIdExist.into())?;
                }
            }
        }
        Ok(()).into()
    }

    pub async fn get_mansion(
        _: MansionAuthentication,
        ReqPretreatment(mid): MidCheckerPretreatment,
        ReqPretreatment(db): MansionMongoDbPretreatment,
    ) -> MansionRResult<ViewMansion> {
        let MansionId { main_id, minor_id } = mid.id;
        let db = db;

        let filter = doc! {
            "id" : {
                "main_id":main_id,
                "minor_id":minor_id as i32
            }
        };
        let data = async_time_usage_with_name(
            "查询Mansion信息",
            db.doing::<_, ModelMansion, _, _>(|collect| {
                async move { collect.find_one(filter, None).await }
            }),
        )
        .await?
        .ok_or(MansionNotFound)?;

        MansionRResult::ok(data.into())
    }

    pub async fn get_recent_id(
        _: MansionAuthentication,
        ReqPretreatment(db): MansionMongoDbPretreatment,
    ) -> MansionRResult<Vec<String>> {
        // 最近60天
        let now = Local::now().naive_local() - (chrono::Duration::days(60));
        let now =
            mongodb::bson::DateTime::from_millis(now.timestamp_millis());
        log::info!("mansion after {:?}", now);
        let filter = doc! {
            "create_time":{
                "$gte":now
            }
        };

        let resp = async_time_usage_with_name(
            "获取MansionID列表（最近60天）",
            db.doing::<_, ModelMansion, _, _>(|collect| {
                async move {
                    let collect = collect.clone_with_type::<Mid>();
                    let mut vec = collect
                        .find(
                            filter,
                            FindOptions::builder()
                                .projection(doc! {"id":1i32})
                                .sort(doc! {"id.main_id":1,"id.minor_id":1})
                                .build(),
                        )
                        .await?;
                    let mut resp = Vec::new();
                    while let Some(v) = vec.next().await {
                        resp.push(v?);
                    }
                    Ok(resp)
                }
            }),
        )
        .await?
        .into_iter()
        .map(|id| id.id.to_string())
        .collect();

        Ok(resp).into()
    }

    pub async fn remove_mansion(
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
        async_time_usage_with_name(
            "移除Mansion",
            db.doing::<_, ModelMansion, _, ()>(|collect| {
                async move {
                    collect.delete_one(filter, None).await?;
                    Ok(())
                }
            }),
        )
        .await?;

        Ok(()).into()
    }
}
