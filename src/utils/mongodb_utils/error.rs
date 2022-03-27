use super::MongoErr;

status_err::status_error! {
    pub MongoDatabaseNotFound[
        status_err::ErrPrefix::MONGO_DB,
        0018
    ]=>"数据库不存在"
}

status_err::status_error! {
    pub MongoDatabaseCollectionNotFound[
        status_err::ErrPrefix::MONGO_DB,
        0019
    ]=>"数据库中集合不存在"
}

crate::error_generate!(
    pub MongoDbError
        Db=MongoErr
        DbNotFound=MongoDatabaseNotFound
        CollectionNotFound=MongoDatabaseCollectionNotFound
);
