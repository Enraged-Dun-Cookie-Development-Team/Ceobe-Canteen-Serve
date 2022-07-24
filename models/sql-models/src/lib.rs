pub mod admin_user;

use std::pin::Pin;

use futures::Stream;
use sea_orm::DbErr;
pub use sql_connection;
pub mod ceobe_operation;

pub type StreamResult<'b, M> =
    Pin<Box<dyn Stream<Item = Result<M, DbErr>> + 'b + Send>>;
