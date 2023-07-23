use chrono::{Local, NaiveDateTime};
use sea_orm::Value;

pub use chrono;

pub fn get_now_naive_date_time_value() -> Value {
    Local::now().naive_local().into()
}

pub fn get_zero_data_time() -> NaiveDateTime {
    NaiveDateTime::from_timestamp_opt(0, 0).unwrap()
}

pub fn get_now_naive_date_time() -> NaiveDateTime {
    Local::now().naive_local()
}