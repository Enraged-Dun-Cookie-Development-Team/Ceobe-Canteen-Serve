
use db_ops_prelude::{mongo_models::ceobe::user::models::UserModel, mongo_connection::MongoDbCollectionTrait};

use super::UserOperate;

impl<'db, Conn> UserOperate<'db, Conn> where
    Conn: MongoDbCollectionTrait<'db, UserModel>
{
}
