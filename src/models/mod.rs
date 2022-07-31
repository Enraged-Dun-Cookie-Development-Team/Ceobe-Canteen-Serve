pub mod sql {
    pub use orm_migrate::sql_models::admin_user::*;
}

pub mod mansion {
    pub use mongo_migration::mongo_models::bakery::mansion::*;
}
