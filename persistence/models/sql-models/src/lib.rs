pub mod admin_user;
pub mod ceobe_cookie;
pub mod ceobe_operation;
pub mod fetcher;

use std::pin::Pin;

use futures::Stream;
use sea_orm::DbErr;
use sql_connection::ext_traits::soft_delete::SoftDelete;
use time_utils::{chrono::NaiveDateTime, get_now_naive_date_time};

pub type StreamResult<'b, M> =
    Pin<Box<dyn Stream<Item = Result<M, DbErr>> + 'b + Send>>;
