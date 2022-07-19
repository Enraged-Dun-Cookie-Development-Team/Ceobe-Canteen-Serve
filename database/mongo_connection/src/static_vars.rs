use once_cell::sync::OnceCell;

use crate::database::manager::DatabaseManage;

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
