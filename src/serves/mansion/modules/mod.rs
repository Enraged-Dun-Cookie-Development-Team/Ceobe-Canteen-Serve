use mongodb::bson::doc;

use crate::utils::mongodb_utils::db_manager::DbBuild;

use self::mansion::Mansion;

pub mod mansion;

crate::db_selector!(pub(super)MansionDb = "mansion-date");

pub(in crate::serves::mansion) fn loading_model(db: &mut DbBuild) {
    db.add_collection_operate::<Mansion, _>(|db, n| db.collection::<Mansion>("mansion-data"));
}
