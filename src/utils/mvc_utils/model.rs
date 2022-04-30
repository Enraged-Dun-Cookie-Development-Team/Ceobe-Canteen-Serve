use crate::{
    utils::mongodb_utils::db_manager::DbBuild,
};

mod mongo;

pub use mongo::{as_mongo_register, MongoRegister};

#[async_trait::async_trait]
#[deprecated]
pub trait ModelRegister: Sized {
    /// 注册 Mongo db 模型
    async fn register_mongo(self, db: DbBuild) -> DbBuild { db }
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
        }
    };
}

async fn temp(db: DbBuild) -> DbBuild { db }

generate_model_register!(
    MockModel,
    as_mongo_register(temp)
);
