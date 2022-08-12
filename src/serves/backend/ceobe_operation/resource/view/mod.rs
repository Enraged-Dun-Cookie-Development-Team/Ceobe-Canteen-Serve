mod countdown;
mod all_available;
use std::borrow::Cow;

use chrono::NaiveDateTime;
use modify_cache::ModifyState;
use orm_migrate::sql_models::{
    ceobe_operation::resource::models::model_resource, get_zero_data_time,
};
use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct Resource {
    resource_all_available: AllAvailable,
    countdown: Vec<Countdown>,
    #[serde(skip)]
    modify_at: NaiveDateTime,
}

pub use all_available::AllAvailable;
pub use countdown::Countdown;

impl ModifyState for Resource {
    type Identify = Self;

    fn get_last_modify_time(&self) -> Option<Cow<'_, NaiveDateTime>> {
        Some(Cow::Borrowed(&self.modify_at))
    }

    fn get_identify(&self) -> Cow<'_, Self::Identify> { Cow::Borrowed(self) }
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
