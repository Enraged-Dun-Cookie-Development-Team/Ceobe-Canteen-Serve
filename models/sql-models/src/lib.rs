pub mod admin_user;

use std::pin::Pin;

use chrono::{Local, NaiveDateTime};
use futures::Stream;
use sea_orm::{DbErr, Value};
pub use sql_connection;
pub mod ceobe_operation;

pub type StreamResult<'b, M> =
    Pin<Box<dyn Stream<Item = Result<M, DbErr>> + 'b + Send>>;

pub fn get_now_naive_date_time() -> Value { Local::now().naive_local().into() }

pub fn get_zero_data_time() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(0, 0)
}
