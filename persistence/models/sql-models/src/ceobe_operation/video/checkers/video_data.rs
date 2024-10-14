use checker::{
    check_gen,
    prefabs::{
        date_time_format::DateTimeFormatChecker,
        str_len_checker::StrMaxCharLenChecker, url_checker::UrlChecker,
    },
};
use chrono::NaiveDateTime;
use sea_orm::{IntoActiveModel, Set};
use sql_connection::ext_traits::ActiveModelUpdater;
use typed_builder::TypedBuilder;
use url::Url;

use super::{
    bv::{Bv, BvChecker},
    CheckError,
};
use crate::{ceobe_operation::video::ActiveModel, SoftDelete};

#[derive(Debug, TypedBuilder)]
pub struct CeobeOpVideo {
    pub bv: Bv,
    pub start_time: NaiveDateTime,
    pub over_time: NaiveDateTime,
    pub title: String,
    pub author: String,
    pub video_link: Url,
    pub cover_image: Url,
}
#[check_gen(
    uncheck = CeobeOpVideoUncheck,
    checked = CeobeOpVideo,
    error = CheckError
)]
#[derive(Debug, serde::Deserialize)]
pub struct CeobeOpVideoChecker {
    pub bv: BvChecker,
    pub start_time: DateTimeFormatChecker,
    pub over_time: DateTimeFormatChecker,
    pub title: StrMaxCharLenChecker<String, 256>,
    pub author: StrMaxCharLenChecker<String, 128>,
    pub video_link: UrlChecker,
    #[serde(alias = "cover_img")]
    pub cover_image: UrlChecker,
}

impl IntoActiveModel<ActiveModel> for CeobeOpVideo {
    fn into_active_model(self) -> ActiveModel {
        let Self {
            bv,
            start_time,
            over_time,
            title,
            author,
            video_link,
            cover_image,
        } = self;
        ActiveModel {
            bv: Set(bv.to_string()),
            start_time: Set(start_time),
            over_time: Set(over_time),
            title: Set(title),
            author: Set(author),
            video_link: Set(video_link.to_string()),
            cover_image: Set(cover_image.to_string()),
            ..Default::default()
        }
    }
}

impl ActiveModelUpdater<ActiveModel> for CeobeOpVideo {
    fn update_active(self, active_model: &mut ActiveModel) {
        let Self {
            start_time,
            over_time,
            title,
            author,
            video_link,
            cover_image,
            ..
        } = self;
        active_model.start_time = Set(start_time);
        active_model.over_time = Set(over_time);
        active_model.title = Set(title);
        active_model.author = Set(author);
        active_model.video_link = Set(video_link.to_string());
        active_model.cover_image = Set(cover_image.to_string());
        active_model.soft_recover();
    }
}
