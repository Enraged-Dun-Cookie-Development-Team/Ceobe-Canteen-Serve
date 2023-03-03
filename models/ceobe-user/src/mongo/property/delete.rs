use db_ops_prelude::{
    mongo_connection::MongoDbCollectionTrait,
    mongo_models::ceobe::user_property::models::UserPropertyModel,
};

use super::UserPropertyOperate;

impl<'db, Conn> UserPropertyOperate<'db, Conn> where
    Conn: MongoDbCollectionTrait<'db, UserPropertyModel>
{
}
