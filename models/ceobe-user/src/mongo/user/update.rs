use std::future::Future;

use db_ops_prelude::{
    mongo_connection::MongoDbCollectionTrait,
    mongo_models::ceobe::user::models::UserModel,
    mongodb::bson::{doc, Uuid},
};
use tracing::{info, instrument};

use super::{OperateResult, UserOperate};

impl<'db, Conn> UserOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, UserModel>,
{
    /// 更新数据源配置
    /// params: mob_id 用户mob id
    ///         datasource_list 更新的数据源
    #[instrument(skip(self), ret)]
    pub fn update_datasource(
        &'db self, mob_id: String, datasource_list: Vec<Uuid>,
    ) -> impl Future<Output = OperateResult<()>> + Send + 'static {
        info!(
            updateDatasource.mob_id = mob_id,
            updateDatasource.datasource_list = ?datasource_list
        );
        let collection = self.get_collection();
        async {
            // 将用户初始化信息存入数据库
            collection?
                .doing(|collection| {
                    collection.update_one(
                        doc! {"mob_id": mob_id},
                        doc! {"$set": {"datasource_push": datasource_list}},
                        None,
                    )
                })
                .await?;
            Ok(())
        }
    }
}
