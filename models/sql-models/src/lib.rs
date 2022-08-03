pub mod admin_user;
pub mod ceobe_operation;

use std::pin::Pin;

use chrono::{Local, NaiveDateTime};
use futures::Stream;
use sea_orm::{DbErr, Value};
pub use sql_connection;

pub type StreamResult<'b, M> =
    Pin<Box<dyn Stream<Item = Result<M, DbErr>> + 'b + Send>>;

pub fn get_now_naive_date_time_value() -> Value {
    Local::now().naive_local().into()
}

pub fn get_zero_data_time() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(0, 0)
}

pub fn get_now_naive_date_time() -> NaiveDateTime {
    Local::now().naive_local()
}

