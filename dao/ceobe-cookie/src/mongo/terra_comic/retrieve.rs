use db_ops_prelude::mongo_connection::MongoDbError;
use db_ops_prelude::{mongo_connection::MongoDbCollectionTrait, mongodb::bson::doc};
use db_ops_prelude::mongo_models::ceobe::cookie::terra_comic::models::TerraComicModel;
use tracing::instrument;
use db_ops_prelude::futures::StreamExt;
use super::{TerraComicOperate, OperateResult};



impl<'db, Conn> TerraComicOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, TerraComicModel>,
{
    /// 分页查询饼数据
    #[instrument(skip(self), ret)]
    pub async fn find_all_comic(
        &'db self
    ) -> OperateResult<Vec<TerraComicModel>> {
        let collection = self.get_collection()?;
        let mut vec = collection
            .doing(|collection| {
                collection.find(
                    None,
                    None
                )
            })
            .await?;
        let mut res = Vec::<TerraComicModel>::new();
        while let Some(v) = vec.next().await {
            res.push(v.map_err(MongoDbError::from)?);
        }
        Ok(res)
    }
}