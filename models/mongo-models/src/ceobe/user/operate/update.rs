use crate::ceobe::user::models::UserModel;

use super::UserOperate;
use mongo_connection::MongoDbCollectionTrait;
use mongodb::bson::{doc, Uuid};
use tracing::{instrument, info};
use crate::ceobe::user::operate::OperateResult;

impl<'db, Conn> UserOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, UserModel>,
{
    /// 更新数据源配置
    /// params: mob_id 用户mob id
    ///         datasource_list 更新的数据源
    #[instrument(skip(self), ret)]
    pub async fn update_datasource(&'db self, mob_id: &str, datasource_list: Vec<Uuid>) -> OperateResult<()> {
        info!(
            updateDatasource.mob_id = mob_id,
            updateDatasource.datasource_list = ?datasource_list
        );

        let collection = self.get_collection()?;

        // 将用户初始化信息存入数据库
        collection
            .doing(|collection| {
                collection.update_one(doc! {"mob_id": mob_id}, doc! {"$set": {"datasource_push":datasource_list}}, None)
            })
            .await
            .map(|_| ())
            .map_err(Into::into)
    }
}