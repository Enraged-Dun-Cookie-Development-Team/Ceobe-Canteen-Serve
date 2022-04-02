use mongodb::{bson::doc, options::IndexOptions, IndexModel};

use crate::{
    serves::mansion::modules::mansion::ModelMansion,
    utils::mongodb_utils::db_manager::DbBuild,
};

pub mod mansion;

const MANSION_IDX_NAME: &str = "mansion_idx";
const MANSION_CREATE_TIME_IDX_NAME: &str = "mansion_create_time_idx";

pub(in crate::serves::mansion) async fn loading_model(
    mut db: DbBuild,
) -> DbBuild {
    db.add_collection_operate(|d, _n| {
        async move {
            let mansion = d.collection::<ModelMansion>("mansion-data");

            let all_idx =
                mansion.list_index_names().await.unwrap_or_default();
            println!(" all idx {:?}", all_idx);
            if !all_idx.contains(&MANSION_IDX_NAME.to_string()) {
                mansion
                    .create_index(
                        IndexModel::builder()
                            .keys(doc! {
                                "id.main_id":1i32,
                                "id.minor_id":1i32,
                            })
                            .options(
                                IndexOptions::builder()
                                    .unique(true)
                                    .name(MANSION_IDX_NAME.to_string())
                                    .build(),
                            )
                            .build(),
                        None,
                    )
                    .await
                    .expect("创建索引时异常");
            }
            if !all_idx.contains(&MANSION_CREATE_TIME_IDX_NAME.to_string()) {
                mansion
                    .create_index(
                        IndexModel::builder()
                            .keys(doc! {
                                "create_time":1i32
                            })
                            .options(
                                IndexOptions::builder()
                                    .name(
                                        MANSION_CREATE_TIME_IDX_NAME
                                            .to_string(),
                                    )
                                    .build(),
                            )
                            .build(),
                        None,
                    )
                    .await
                    .expect("创建索引时异常");
            }

            mansion
        }
    })
    .await;

    db
}
