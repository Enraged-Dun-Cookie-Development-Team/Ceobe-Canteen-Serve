use std::borrow::Cow;

use chrono::Local;
use modify_cache::ModifyState;
use mongodb::bson::{doc, DateTime, Document, Uuid};
use serde::{Deserialize, Serialize};
use sub_model::SubModel;
use typed_builder::TypedBuilder;

use crate::{RecordUnit, RecordUnitUpdater, SetRecordUnit};

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, SubModel)]
#[sub_model(
    all(
        vis = "pub",
        name = "UserPropertyChecked",
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
pub struct UserPropertyModel {
    #[sub_model(want("UserMobId"))]
    pub mob_id: String,
    #[sub_model(want("UserDatasource"))]
    pub datasource_push: Vec<Uuid>,
    #[sub_model(ignore("UserPropertyChecked"))]
    pub last_access_time: DateTime,
    #[sub_model(ignore("UserPropertyChecked"))]
    pub time_record: RecordUnit,
}

impl ModifyState for UserPropertyModel {
    type Identify = Self;

    fn get_last_modify_time(&self) -> Option<Cow<'_, chrono::NaiveDateTime>> {
        Some(Cow::Owned(
            self.time_record.modify_at.to_chrono().naive_local(),
        ))
    }

    fn get_identify(&self) -> Cow<'_, Self::Identify> { Cow::Borrowed(self) }
}

impl UserPropertyModel {
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

impl From<UserPropertyChecked> for UserPropertyModel {
    fn from(user: UserPropertyChecked) -> Self {
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

impl RecordUnitUpdater for UserPropertyModel {
    type Source = UserPropertyChecked;

    fn get_mut(&mut self) -> &mut RecordUnit { &mut self.time_record }
}

impl SetRecordUnit for UserPropertyChecked {
    type Target = UserPropertyModel;

    fn into_with_time_record(self, time_record: RecordUnit) -> Self::Target {
        let Self {
            mob_id,
            datasource_push,
        } = self;

        Self::Target {
            mob_id,
            datasource_push,
            last_access_time: time_record.create_at,
            time_record,
        }
    }
}
