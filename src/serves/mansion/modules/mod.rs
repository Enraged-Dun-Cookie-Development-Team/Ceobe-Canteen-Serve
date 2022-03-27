use crate::utils::mongodb_utils::db_manager::DbBuild;

use self::mansion::Mansion;

pub mod mansion;

crate::db_selector!(pub(in crate::serves::mansion)MansionDb = "ceobe-canteen");

pub(in crate::serves::mansion) async fn loading_model(mut db: DbBuild)->DbBuild {
    db.add_collection_operate::<Mansion, _,_>(|d, _n| async move {
        d.collection::<Mansion>("mansion-data")
    })
    .await;

    db
}
