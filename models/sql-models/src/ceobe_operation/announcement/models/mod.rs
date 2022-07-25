use chrono::{Local, NaiveDateTime};

pub mod model_announcement;

pub(in crate::ceobe_operation::announcement) fn get_now_naive_date_time(
) -> Option<Box<NaiveDateTime>> {
    Box::new(Local::now().naive_local()).into()
}

pub(in crate::ceobe_operation::announcement) fn get_zero_data_time(
) -> NaiveDateTime {
    NaiveDateTime::from_timestamp(0, 0)
}
