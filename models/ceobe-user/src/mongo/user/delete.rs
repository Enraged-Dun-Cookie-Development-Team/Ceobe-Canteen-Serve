use db_ops_prelude::{
    mongo_connection::MongoDbCollectionTrait,
    mongo_models::ceobe::user_property::models::UserPropertyModel,
};

use super::UserOperate;

impl<'db, Conn> UserOperate<'db, Conn> where
    Conn: MongoDbCollectionTrait<'db, UserPropertyModel>
{
}
