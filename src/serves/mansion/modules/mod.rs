use mongodb::{bson::doc, options::IndexOptions, IndexModel};

use crate::utils::mongodb_utils::db_manager::DbBuild;

use self::mansion::Mansion;

pub mod mansion;

const MANSION_IDX_NAME: &str = "mansion_idx";

pub(in crate::serves::mansion) async fn loading_model(mut db: DbBuild) -> DbBuild {
    db.add_collection_operate::<Mansion, _, _>(|d, _n| async move {
        let mansion = d.collection::<Mansion>("mansion-data");

        let all_idx = mansion.list_index_names().await.expect("无法获取异常信息");
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

        mansion
    })
    .await;

    db
}
