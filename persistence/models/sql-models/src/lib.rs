pub mod admin_user;
pub mod ceobe_cookie;
pub mod ceobe_operation;
pub mod fetcher;

use std::pin::Pin;
use futures::Stream;
use sea_orm::DbErr;
use time_utils::{get_now_naive_date_time,chrono::NaiveDateTime};
use sql_connection::ext_traits::soft_delete::SoftDelete;

pub type StreamResult<'b, M> =
Pin<Box<dyn Stream<Item = Result<M, DbErr>> + 'b + Send>>;
