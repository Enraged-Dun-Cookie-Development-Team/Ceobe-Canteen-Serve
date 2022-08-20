use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use crate::{
    database::manager::DatabaseManage, CollectionGuard, MongoDbError,
};

static MONGO_DATABASE_CONNECTION: OnceCell<DatabaseManage> = OnceCell::new();

pub(crate) fn set_mongo_database(db: DatabaseManage) {
    if MONGO_DATABASE_CONNECTION.set(db).is_err() {
        panic!("Mongo数据库连接已经建立")
    }
}

pub fn get_mongo_database() -> &'static DatabaseManage {
    MONGO_DATABASE_CONNECTION
        .get()
        .expect("Mongo数据库连接未建立")
}

pub fn get_mongo_collection<
    C: Serialize + for<'de> Deserialize<'de> + 'static,
>() -> Result<CollectionGuard<C>, MongoDbError> {
    get_mongo_database().get_collection::<C>()
}
