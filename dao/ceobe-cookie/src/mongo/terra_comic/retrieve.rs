use db_ops_prelude::{
    futures::StreamExt,
    mongo_connection::{MongoDbCollectionTrait, MongoDbError},
    mongo_models::ceobe::cookie::terra_comic::models::TerraComicModel,
    mongodb::bson::doc,
};
use tracing::instrument;

use super::{OperateResult, TerraComicOperate};

impl<'db, Conn> TerraComicOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, TerraComicModel>,
{
    /// 查询所有漫画
    #[instrument(skip(self), ret)]
    pub async fn find_all_comic(
        &'db self,
    ) -> OperateResult<Vec<TerraComicModel>> {
        let collection = self.get_collection()?;
        let mut vec = collection
            .doing(|collection| collection.find(None, None))
            .await?;
        let mut res = Vec::<TerraComicModel>::new();
        while let Some(v) = vec.next().await {
            res.push(v.map_err(MongoDbError::from)?);
        }
        Ok(res)
    }

     /// 查询指定漫画信息
     #[instrument(skip(self), ret)]
     pub async fn find_comic_by_id(
         &'db self, comic_id: &str
     ) -> OperateResult<Option<TerraComicModel>> {
         let collection = self.get_collection()?;
         let res = collection
             .doing(|collection| collection.find_one(doc! {"_id": comic_id}, None))
             .await?;
         Ok(res)
     }
}
