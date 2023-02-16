use std::borrow::Cow;

use chrono::Local;
use modify_cache::ModifyState;
use mongodb::bson::{doc, DateTime, Document, Uuid};
use serde::{Deserialize, Serialize};
use sub_model::SubModel;
use typed_builder::TypedBuilder;

use crate::{RecordUnit, RecordUnitSet};

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
    // 用户进行活跃度刷新操作
    pub fn user_access(mut self) -> Self {
        let now = Local::now();

        self.last_access_time = DateTime::from_chrono(now);
        self
    }
}

impl From<UserChecked> for UserModel {
    fn from(user: UserChecked) -> Self {
        Self::into_with_time_record(user, RecordUnit::new())
    }
}

impl UserMobId {
    pub fn into_id_filter(&self) -> Document {
        doc! {
            "mob_id" : &self.mob_id
        }
    }
}

impl RecordUnitSet for UserModel {
    type Source = UserChecked;

    fn get_mut(&mut self) -> &mut RecordUnit { &mut self.time_record }

    fn into_with_time_record(
        model: Self::Source, time_record: RecordUnit,
    ) -> Self {
        let Self::Source {
            mob_id,
            datasource_push,
        } = model;

        UserModel {
            mob_id,
            datasource_push,
            last_access_time: time_record.create_at,
            time_record,
        }
    }
}
