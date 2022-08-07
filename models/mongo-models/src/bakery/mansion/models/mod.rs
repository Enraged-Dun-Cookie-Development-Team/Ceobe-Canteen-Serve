use std::borrow::Cow;

use modify_cache::ModifyState;
use chrono::Local;
use mongodb::bson::{self, doc, Document};
use serde::{Deserialize, Serialize};
use sub_model::SubModel;
use typed_builder::TypedBuilder;

use super::checked::{Daily, Mansion};

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct MansionId {
    pub main_id: u32,
    pub minor_id: u8,
}
#[derive(SubModel)]
#[sub_model(none(
    name = "ModifyAt",
    extra(derive(serde::Serialize, serde::Deserialize))
))]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ModelMansion {
    /// create record
    #[sub_model(want("ModifyAt"))]
    pub create_time: bson::DateTime,
    /// modify time
    #[sub_model(want("ModifyAt"))]
    pub modify_time: bson::DateTime,
    // old fields
    pub id: MansionId,
    pub description: String,
    pub cvlink: String,
    pub fraction: u8,
    pub daily: Vec<Daily>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum Predict {
    #[serde(rename = "false")]
    False,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "true")]
    True,
}

impl From<Mansion> for ModelMansion {
    fn from(m: Mansion) -> Self {
        Self::with_modify_time(m, Default::default())
    }
}

impl ModifyState for ModelMansion {
    type Identify = Self;

    fn get_last_modify_time(&self) -> Option<Cow<'_, chrono::NaiveDateTime>> {
        Some(Cow::Owned(self.modify_time.to_chrono().naive_local()))
    }

    fn get_identify(&self) -> Cow<'_, Self::Identify> { Cow::Borrowed(self) }
}

impl ModelMansion {
    pub fn with_modify_time(
        Mansion {
            id,
            link: cvlink,
            description,
            fraction,
            daily,
        }: Mansion,
        ModifyAt {
            create_time,
            modify_time,
        }: ModifyAt,
    ) -> Self {
        Self {
            create_time,
            modify_time,
            id,
            description,
            cvlink,
            fraction: fraction as u8,
            daily,
        }
    }
}

impl Default for ModifyAt {
    fn default() -> Self {
        let now = bson::DateTime::from_millis(
            Local::now().naive_local().timestamp_millis(),
        );
        Self {
            create_time: now,
            modify_time: now,
        }
    }
}

impl ModifyAt {
    pub fn now_modify(mut self) -> Self {
        self.modify_time = bson::DateTime::from_millis(
            Local::now().naive_local().timestamp_millis(),
        );
        self
    }
}

impl std::fmt::Display for MansionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.main_id, self.minor_id)
    }
}

impl MansionId {
    pub fn into_id_filter(&self) -> Document {
        doc! {
            "id" : {
                "main_id":self.main_id,
                "minor_id":self.minor_id as i32
            }
        }
    }
}
