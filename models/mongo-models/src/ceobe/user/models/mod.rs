use std::borrow::Cow;

use modify_cache::ModifyState;
use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};
use sub_model::SubModel;
use typed_builder::TypedBuilder;

use crate::RecordUnit;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder, SubModel)]
#[sub_model(all(
    vis = "pub",
    name = "UserChecked",
    extra(derive(Debug, TypedBuilder))
))]
pub struct UserModel {
    pub mob_id: String,
    pub datasource_push: Vec<Uuid>,
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
}

impl From<UserChecked> for UserModel {
    fn from(user: UserChecked) -> Self {
        user.into_with_time_record(RecordUnit::new())
    }
}
