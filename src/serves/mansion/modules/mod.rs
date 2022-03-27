use mongodb::{bson::doc, options::IndexOptions, IndexModel};

use crate::utils::mongodb_utils::db_manager::DbBuild;

use self::mansion::Mansion;

pub mod mansion;

crate::db_selector!(pub(in crate::serves::mansion)MansionDb = "ceobe-canteen");

pub(in crate::serves::mansion) async fn loading_model(mut db: DbBuild) -> DbBuild {
    db.add_collection_operate::<Mansion, _, _>(|d, _n| async move {
        let mansion = d.collection::<Mansion>("mansion-data");
        mansion
            .create_index(
                IndexModel::builder()
                    .keys(doc! {
                        "id.main_id":1i32,
                        "id.minor_id":1i32,
                    })
                    .options(IndexOptions::builder().unique(true).build())
                    .build(),
                None,
            )
            .await
            .expect("创建索引时异常");

        mansion
    })
    .await;

    db
}
