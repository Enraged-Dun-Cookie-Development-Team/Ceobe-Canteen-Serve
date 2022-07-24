use chrono::NaiveDateTime;

pub mod model_announcement;


pub(in crate::ceobe_operation::announcement) fn get_zero_data_time() -> NaiveDateTime
{
    NaiveDateTime::from_timestamp(0, 0)
}
