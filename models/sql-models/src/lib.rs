pub mod admin_user;
pub mod ceobe_operation;
pub mod fetcher;

use std::pin::Pin;

use chrono::{Local, NaiveDateTime};
use futures::Stream;
use sea_orm::{DbErr, Value, Set, ActiveValue};
pub use sql_connection;
use sea_orm::prelude::DateTime;

pub type StreamResult<'b, M> =
    Pin<Box<dyn Stream<Item = Result<M, DbErr>> + 'b + Send>>;

pub fn get_now_naive_date_time_value() -> Value {
    Local::now().naive_local().into()
}

pub fn get_zero_data_time() -> NaiveDateTime {
    NaiveDateTime::from_timestamp_opt(0, 0).unwrap()
}

pub fn get_now_naive_date_time() -> NaiveDateTime {
    Local::now().naive_local()
}

pub trait SoftDelete {
    fn get_mut(&mut self) -> &mut ActiveValue<NaiveDateTime> ;

    fn mut_by(&mut self, f: impl FnOnce(&mut ActiveValue<NaiveDateTime> )) {
        f(self.get_mut())
    }

    fn soft_remove(&mut self) {
        self.mut_by(|delete| {
            *delete = Set(get_now_naive_date_time());
        })
    }

    fn soft_recover(&mut self) {
        self.mut_by(|delete| {
            *delete = Set(get_zero_data_time());
        })
    }
}
