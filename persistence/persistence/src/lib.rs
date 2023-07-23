#[cfg(feature = "model-admin")]
pub mod admin {
    pub use dao_admin::*;
    pub use sql_models::admin_user as models;
}

#[cfg(feature = "model-bakery")]
pub mod bakery {
    pub use dao_bakery::*;
    pub use mongo_models::bakery as models;
}

#[cfg(feature = "model-ceobe-cookie")]
pub mod ceobe_cookie {
    pub use dao_ceobe_cookie::*;

    pub mod models {
        pub use mongo_models::ceobe::cookie::*;
        pub use sql_models::ceobe_cookie::*;
    }
}

#[cfg(feature = "model-ceobe-operate")]
pub mod ceobe_operate {
    pub use dao_ceobe_operate::*;

    pub mod models {
        pub use mongo_models::ceobe::operation::*;
        pub use sql_models::ceobe_operation::*;
    }
}

#[cfg(feature = "model-ceobe-user")]
pub mod ceobe_user {
    pub use dao_ceobe_user::*;
    pub use mongo_models::ceobe::user_property as models;
}

/// prelude export
#[cfg(feature = "help-crates")]
pub mod help_crates {
    pub use bool_or;
    pub use futures;
    pub use mongodb;
    pub use sea_orm;
    pub use smallstr;
    pub use smallvec;
    pub use status_err::{ErrPrefix, HttpCode, StatusErr};
    pub use tap;
    pub use thiserror::Error as ThisError;
    pub use tracing;
    pub use time_utils::*;
}

#[cfg(any(feature = "model-fetcher", feature = "prelude"))]
pub mod fetcher {
    pub use dao_fetcher::*;
    pub use sql_models::fetcher as models;
}

#[cfg(any(feature = "mongo", feature = "mongo-migrate", ))]
pub mod mongodb {
    pub use mongo_connect::*;
    #[cfg(feature = "mongo-migrate")]
    pub use mongo_migration::*;
}

#[cfg(any(feature = "mysql", feature = "mysql-migrate", ))]
pub mod mysql {
    #[cfg(feature = "help-crates")]
    pub use mysql_func;
    pub use sql_connect::*;
    #[cfg(feature = "sql-migration")]
    pub use sql_migration::*;
}

#[cfg(feature = "redis")]
pub mod redis {
    pub use redis_connect::*;
}

#[cfg(feature = "database-connect")]
pub mod connect {
    pub use traits::initial::*;
}

#[cfg(feature = "database-operate")]
pub mod operate {
    pub use r#abstract::*;
    pub use traits::{database_operates::*, get_connect::*};
}
