use tracing::instrument;

use db_ops_prelude::mongo_connection::MongoDbCollectionTrait;
use db_ops_prelude::mongodb::bson::{doc, Uuid};

use crate::tool_link_mongodb::models::ToolLink;

use super::{OperateResult, ToolLinkOperate};

impl<'db, Conn> ToolLinkOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, ToolLink>,
{
    #[instrument(skip(self), name = "list")]
    pub async fn delete(
        &'db self,
        id: String,
    ) -> OperateResult<()> {
        let db = self.get_collection()?;

        db.doing(|collection| {
            collection.delete_one(
                doc! {"id": Uuid::parse_str(id).unwrap()},
                None,
            )
        })
            .await?;

        Ok(())
    }
}
