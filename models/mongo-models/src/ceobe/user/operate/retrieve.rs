use super::UserOperate;
use crate::ceobe::user::models::UserMobId;
use crate::ceobe::user::models::UserModel;
use crate::ceobe::user::operate::OperateResult;
use mongo_connection::CollectionGuard;
use mongo_connection::MongoDbCollectionTrait;
use mongodb::bson::Document;
use mongodb::bson::Uuid;
use mongodb::bson::doc;
use mongodb::options::FindOneOptions;
use mongodb::options::FindOptions;
use tap::Tap;
use tracing::info;
use crate::ceobe::user::models::UserDatasource;
use tracing::instrument;

impl<'db, Conn> UserOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, UserModel>,
{
    /// 根据条件查询数据源配置
    pub async fn find_datasource_list_by_fiter(
        filter: impl Into<Option<Document>>,
        collection: &CollectionGuard<UserDatasource>,
    ) -> OperateResult<Vec<Uuid>> {
       Ok(collection
            .doing(|collection| async move {
                let datasource_uuids = collection
                    .find_one(
                        filter,
                        FindOneOptions::builder()
                            .projection(doc! {"datasource_push":1i32})
                            .build(),
                    )
                    .await?;
                    let mut res = Vec::new();
                    if let Some(info) = datasource_uuids {
                        res = info.datasource_push
                    }
                    Ok(res)
            })
            .await?
            .tap(|list| info!(mansionList.ids = ?list))
        )  
    }
}
