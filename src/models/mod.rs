crate::generate_model_register!(RootModels);

pub mod common {
    pub mod sql {
        pub use orm_migrate::sql_models::common::*;
    }
}

pub mod mansion {
    pub use mongo_migration::mongo_models::mansion::*;
}
