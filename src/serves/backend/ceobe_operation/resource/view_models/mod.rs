use std::borrow::Cow;

use chrono::NaiveDateTime;
use modify_cache::ModifyState;
use orm_migrate::sql_models::{
    ceobe_operation::resource::models::model_resource, get_zero_data_time,
};
use serde::Serialize;

use crate::utils::time_format::naive_date_time_format;

#[derive(Debug, Clone, Serialize)]
pub struct Resource {
    resource_all_available: AllAvailable,
    countdown: Vec<Countdown>,
    #[serde(skip)]
    modify_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize)]
pub struct AllAvailable {
    start_time: String,
    over_time: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Countdown {
    #[serde(alias = "text")]
    message: String,
    #[serde(alias = "remark")]
    banner_info: String,
    #[serde(alias = "time")]
    countdown_end: String,
    start_time: String,
    over_time: String,
}

impl ModifyState for Resource {
    type Identify = Self;

    fn get_last_modify_time(&self) -> Option<Cow<'_, NaiveDateTime>> {
        Some(Cow::Borrowed(&self.modify_at))
    }

    fn get_identify(&self) -> Cow<'_, Self::Identify> { Cow::Borrowed(self) }
}

impl From<model_resource::Countdown> for Countdown {
    fn from(
        model_resource::Countdown {
            start_time,
            message,
            countdown_end,
            banner_info,
            over_time,
            ..
        }: model_resource::Countdown,
    ) -> Self {
        Self {
            message,
            banner_info,
            countdown_end: naive_date_time_format(countdown_end),
            start_time: naive_date_time_format(start_time),
            over_time: naive_date_time_format(over_time),
        }
    }
}

impl From<model_resource::ResourceAllAvailable> for AllAvailable {
    fn from(
        model_resource::ResourceAllAvailable {
            over_time,
            start_time,
            ..
        }: model_resource::ResourceAllAvailable,
    ) -> Self {
        Self {
            start_time: naive_date_time_format(start_time),
            over_time: naive_date_time_format(over_time),
        }
    }
}

impl
    From<(
        model_resource::ResourceAllAvailable,
        Vec<model_resource::Countdown>,
    )> for Resource
{
    fn from(
        (raa, cd): (
            model_resource::ResourceAllAvailable,
            Vec<model_resource::Countdown>,
        ),
    ) -> Self {
        let modify_at = NaiveDateTime::max(
            raa.modify_at,
            cd.iter()
                .map(|v| v.modify_at)
                .max()
                .unwrap_or_else(get_zero_data_time),
        );

        Self {
            resource_all_available: raa.into(),
            countdown: cd.into_iter().map(Into::into).collect(),
            modify_at,
        }
    }
}
