use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::{
    models::sql::announcement::models::model_announcement,
    utils::time_format::naive_date_time_format,
};

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct AnnouncementItem {
    pub start_time: String,
    pub over_time: String,
    pub content: String,
    pub img_url: String,
    pub notice: bool,
}

impl From<model_announcement::Model> for AnnouncementItem {
    fn from(
        model_announcement::Model {
            start_time,
            over_time,
            content,
            img_url,
            notice,
            ..
        }: model_announcement::Model,
    ) -> Self {
        Self {
            start_time: naive_date_time_format(start_time),
            over_time: naive_date_time_format(over_time),
            content,
            img_url,
            notice,
        }
    }
}
