use crate::{
    database::model_register::SqlModelRegister,
    utils::mongodb_utils::db_manager::DbBuild,
};

mod mongo;
mod sql;

pub use mongo::{as_mongo_register, MongoRegister};
pub use sql::{as_sql_register, SqlRegister};

#[async_trait::async_trait]
pub trait ModelRegister: Sized {
    /// 注册 Mongo db 模型
    async fn register_mongo(self, db: DbBuild) -> DbBuild { db }

    fn register_sql(self, db: SqlModelRegister) -> SqlModelRegister { db }
}

#[macro_export]
macro_rules! generate_model_register {
    ($name:ident $(,$model:expr)*) => {
        pub struct $name;
        #[async_trait::async_trait]
        impl $crate::utils::mvc_utils::ModelRegister for $name{
            async fn register_mongo(self,db:$crate::utils::mongodb_utils::db_manager::DbBuild) ->
             $crate::utils::mongodb_utils::db_manager::DbBuild{
                $(
                    let db = $crate::utils::mvc_utils::ModelRegister::register_mongo($model,db).await;
                )*
                db
            }

            fn register_sql(self, db: $crate::database::model_register::SqlModelRegister) -> $crate::database::model_register::SqlModelRegister {
                $(
                    let db = $crate::utils::mvc_utils::ModelRegister::register_sql($model,db);
                )*
                db
             }
        }
    };
}

async fn temp(db: DbBuild) -> DbBuild { db }

fn temp2(db: SqlModelRegister) -> SqlModelRegister { db }

generate_model_register!(
    MockModel,
    as_mongo_register(temp),
    as_sql_register(temp2)
);
