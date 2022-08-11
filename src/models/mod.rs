pub mod sql {
    pub use orm_migrate::sql_models::admin_user::*;
    pub use orm_migrate::sql_models::ceobe_operation::*;
}

pub mod mongo {
    pub use mongo_migration::mongo_models::bakery::*;
    pub use mongo_migration::mongo_models::ceobe_operation::*;
}
