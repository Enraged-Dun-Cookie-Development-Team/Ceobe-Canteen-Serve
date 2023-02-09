use crate::ceobe::user::models::UserModel;

use super::UserOperate;
use mongo_connection::MongoDbCollectionTrait;

impl<'db, Conn> UserOperate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, UserModel>,
{

}