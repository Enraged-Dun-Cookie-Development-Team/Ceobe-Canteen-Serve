use mongo_connection::MongoDbCollectionTrait;

use super::UserOperate;
use crate::ceobe::user::models::UserModel;

impl<'db, Conn> UserOperate<'db, Conn> where
    Conn: MongoDbCollectionTrait<'db, UserModel>
{
}
