use futures::StreamExt;
use mongodb::{bson::doc, options::FindOptions};
use time_usage::async_time_usage_with_name;

use super::{super::MansionRResult, MansionMongoDbPretreatment};
use crate::{
    models::mansion::preludes::*,
    router::BakeryMansionFrontend,
    serves::mansion::{
        controllers::MidCheckerPretreatment, error::MansionNotFound,
        view::ViewMansionWithTime,
    },
    utils::req_pretreatment::ReqPretreatment,
};

impl BakeryMansionFrontend {
    pub async fn get_mansion_with_time(
        ReqPretreatment(mid): MidCheckerPretreatment,
        ReqPretreatment(db): MansionMongoDbPretreatment,
    ) -> MansionRResult<ViewMansionWithTime> {
        let MansionId { main_id, minor_id } = mid.id;
        let db = db;

        let filter = doc! {
            "id" : {
                "main_id":main_id
                ,
                "minor_id":minor_id as i32
            }
        };
        let data = async_time_usage_with_name(
            "前台：查询Mansion信息",
            db.doing::<_, ModelMansion, _, _>(|collect| {
                async move { collect.find_one(filter, None).await }
            }),
        )
        .await?
        .ok_or(MansionNotFound)?;

        MansionRResult::ok(data.into())
    }

    pub async fn get_all_id(
        ReqPretreatment(db): MansionMongoDbPretreatment,
    ) -> MansionRResult<Vec<String>> {
        let resp = async_time_usage_with_name(
            "前台：获取全部的MansionID列表",
            db.doing::<_, ModelMansion, _, _>(|collect| {
                async move {
                    let collect = collect.clone_with_type::<Mid>();
                    let mut vec = collect
                        .find(
                            None,
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
}
