pub mod admin_user;
pub mod ceobe_operation;

use std::pin::Pin;

use futures::Stream;
use sea_orm::DbErr;
pub use sql_connection;

pub type StreamResult<'b, M> =
    Pin<Box<dyn Stream<Item = Result<M, DbErr>> + 'b + Send>>;
