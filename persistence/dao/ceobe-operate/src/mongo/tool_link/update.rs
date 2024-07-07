use db_ops_prelude::{
    mongo_connection::MongoDbCollectionTrait,
    mongodb::bson::{self, doc},
};
use tracing::instrument;

use super::{OperateResult, ToolLinkOperate};
use crate::tool_link_mongodb::models::ToolLink;

impl<'db, Conn> ToolLinkOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, ToolLink>,
{
    #[instrument(skip(self), name = "list")]
    pub async fn update(&'db self, tool_link: ToolLink) -> OperateResult<()> {
        let db = self.get_collection()?;

        db.doing(|collection| {
            collection.update_one(
                doc! {"id": tool_link.id},
                doc! {
                    "$set": {
                        "localized_name": bson::to_bson(&tool_link.localized_name).unwrap(),
                        "localized_description": bson::to_bson(&tool_link.localized_description).unwrap(),
                        "localized_slogen": bson::to_bson(&tool_link.localized_slogen).unwrap(),
                        "localized_tags": bson::to_bson(&tool_link.localized_tags).unwrap(),
                        "icon_url": tool_link.icon_url,
                        "links": bson::to_bson(&tool_link.links).unwrap(),
                    }
                },
                None,
            )
        })
            .await?;

        Ok(())
    }
}
