use db_ops_prelude::{
    mongo_connection::MongoDbCollectionTrait,
    mongo_models::ceobe::operation::version::models::ReleaseVersion,
};

impl<'db, Conn> super::ReleaseVersionDelete<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, ReleaseVersion>,
{
    
}
