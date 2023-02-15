use std::borrow::Cow;

use chrono::Local;
use modify_cache::ModifyState;
use mongodb::bson::{doc, DateTime, Document, Uuid};
use serde::{Deserialize, Serialize};
use sub_model::SubModel;
use typed_builder::TypedBuilder;

use crate::RecordUnit;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, SubModel)]
#[sub_model(
    all(
        vis = "pub",
        name = "UserChecked",
        extra(derive(Debug, TypedBuilder))
    ),
    none(
        vis = "pub",
        name = "UserMobId",
        extra(derive(Debug, TypedBuilder))
    ),
    none(
        vis = "pub",
        name = "UserDatasource",
        extra(derive(Debug, Clone, Serialize, Deserialize, TypedBuilder))
    )
)]
pub struct UserModel {
    #[sub_model(want("UserMobId"))]
    pub mob_id: String,
    #[sub_model(want("UserDatasource"))]
    pub datasource_push: Vec<Uuid>,
    #[sub_model(ignore("UserChecked"))]
    pub last_access_time: DateTime,
    #[sub_model(ignore("UserChecked"))]
    pub time_record: RecordUnit,
}

impl UserChecked {
    pub fn into_with_time_record(self, time_record: RecordUnit) -> UserModel {
        let Self {
            mob_id,
            datasource_push,
        } = self;

        UserModel {
            mob_id,
            datasource_push,
            last_access_time: time_record.create_at,
            time_record,
        }
    }
}

impl ModifyState for UserModel {
    type Identify = Self;

    fn get_last_modify_time(&self) -> Option<Cow<'_, chrono::NaiveDateTime>> {
        Some(Cow::Owned(
            self.time_record.modify_at.to_chrono().naive_local(),
        ))
    }

    fn get_identify(&self) -> Cow<'_, Self::Identify> { Cow::Borrowed(self) }
}

impl UserModel {
    pub fn now_modify(mut self) -> Self {
        self.time_record.modify();
        self
    }

    // 用户进行活跃度刷新操作
    pub fn user_access(mut self) -> Self {
        let now = Local::now();

        self.last_access_time = DateTime::from_chrono(now);
        self
    }
}

impl From<UserChecked> for UserModel {
    fn from(user: UserChecked) -> Self {
        user.into_with_time_record(RecordUnit::new())
    }
}

impl UserMobId {
    pub fn into_id_filter(&self) -> Document {
        doc! {
            "mob_id" : &self.mob_id
        }
    }
}
