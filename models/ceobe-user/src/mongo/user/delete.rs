use db_ops_prelude::{
    mongo_connection::MongoDbCollectionTrait,
    mongo_models::ceobe::user::models::UserModel,
};

use super::UserOperate;

impl<'db, Conn> UserOperate<'db, Conn> where
    Conn: MongoDbCollectionTrait<'db, UserModel>
{
}
