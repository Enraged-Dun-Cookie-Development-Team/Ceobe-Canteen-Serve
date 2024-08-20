use db_ops_prelude::{
    mongo_connection::MongoDbCollectionTrait,
    mongodb::bson::{doc, Uuid},
};
use tracing::instrument;

use super::{OperateResult, ToolLinkOperate};
use crate::tool_link_mongodb::models::ToolLink;

impl<'db, Conn> ToolLinkOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, ToolLink>,
{
    #[instrument(skip(self))]
    pub async fn delete(&'db self, id: Uuid) -> OperateResult<()> {
        let db = self.get_collection()?;

        db.doing(|collection| collection.delete_one(doc! {"id": id}, None))
            .await?;

        Ok(())
    }
}
