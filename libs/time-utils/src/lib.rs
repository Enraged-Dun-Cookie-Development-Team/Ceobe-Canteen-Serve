pub use chrono;
use chrono::{DateTime, Local, NaiveDateTime};

#[cfg(feature = "with-sea-orm")]
pub fn get_now_naive_date_time_value() -> sea_orm::Value {
    Local::now().naive_local().into()
}

pub fn get_zero_data_time() -> NaiveDateTime {
    DateTime::from_timestamp(0, 0).unwrap().naive_local()
}

pub fn get_now_naive_date_time() -> NaiveDateTime {
    Local::now().naive_local()
}

#[cfg(feature = "with-mongo")]
pub fn now() -> mongodb::bson::DateTime {
    let now = Local::now();
    mongodb::bson::DateTime::from_chrono(now)
}
