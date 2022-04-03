use futures::Future;

use crate::utils::mongodb_utils::db_manager::DbBuild;

#[async_trait::async_trait]
pub trait ModelRegister: Sized {
    /// 注册
    async fn register_mongo(self, db: DbBuild) -> DbBuild { db }
}

#[macro_export]
macro_rules! generate_model_register {
    ($name:ident $(,$routes:path)*) => {
        pub struct $name;
        #[async_trait::async_trait]
        impl $crate::utils::mvc_utils::ModelRegister for $name{
            async fn register_mongo(self,db:$crate::utils::mongodb_utils::db_manager::DbBuild) ->
             $crate::utils::mongodb_utils::db_manager::DbBuild{
                $(
                    let db = $routes.register_mongo(db).await;
                )*
                db
            }
        }
    };
}

#[async_trait::async_trait]
impl<F, Fut> ModelRegister for F
where
    F: FnOnce(DbBuild) -> Fut + Send,
    Fut: Future<Output = DbBuild> + Send,
{
    async fn register_mongo(self, db: DbBuild) -> DbBuild { self(db).await }
}

async fn temp(db: DbBuild) -> DbBuild { db }

generate_model_register!(MockModel, temp);
