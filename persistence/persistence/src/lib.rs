#[cfg(feature = "model-admin")]
pub use admin;
#[cfg(feature = "model-bakery", )]
pub use bakery;
#[cfg(feature = "model-ceobe-cookie", )]
pub use ceobe_cookie;
#[cfg(feature = "model-ceobe-operate",)]
pub use ceobe_operate;
#[cfg(feature = "model-ceobe-user",)]
pub use ceobe_user;
/// prelude export
#[cfg(feature = "prelude")]
pub use db_prelude;
#[cfg(any(feature = "model-fetcher", feature = "prelude"))]
pub use fetcher;

#[cfg(any(feature = "sql-models",feature = "mongo-models"))]
pub mod models{
    #[cfg(all(feature = "sql-models" ,not(feature = "mongo-models")))]
    pub use sql_models::*;
    #[cfg(all(feature = "mongo-models" ,not(feature = "sql-models")))]
    pub use mongo_models::*;
    #[cfg(all(feature = "sql-models",feature = "mongo-models"))]
    pub use {sql_models as mysql,mongo_models as mongo};
}

#[cfg(any(feature = "mongodb", feature = "mongo-migrate", ))]
pub mod mongodb {
    pub use mongo_connect::*;
    #[cfg(feature = "mongo-migrate")]
    pub use mongo_migration::*;
}

#[cfg(any(feature = "mysql", feature = "mysql-migrate", ))]
pub mod mysql {
    pub use sql_connect::*;
    #[cfg(feature = "sql-migration")]
    pub use sql_migration::*;
}

#[cfg(feature = "redis")]
pub mod redis {
    pub use redis_connect::*;
}

#[cfg(feature = "database-connect")]
pub mod connect{
    pub use traits::initial::*;
}
#[cfg(feature = "database-operate")]
pub mod operate{
    pub use traits::database_operates::*;
    pub use traits::get_connect::*;
    pub use r#abstract::*;
}

