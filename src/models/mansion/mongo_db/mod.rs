use chrono::Local;
use mongodb::bson;

use super::check::{Daily, Mansion};

crate::quick_struct! {
    pub MansionId{
        main_id:u32
        minor_id:u8
    }

    pub ModelMansion{
        /// create record
        create_time:bson::DateTime
        /// modify time
        modify_time:Option<bson::DateTime>

        //old fields
        id:MansionId
        description:String
        cvlink:String
        fraction:u8
        daily:Vec<Daily>
    }

    pub ModifyAt{
        create_time:bson::DateTime
        modify_time:Option<bson::DateTime>
    }

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
        Self::builder().create_time(now).modify_time(None).build()
    }
}

impl ModifyAt {
    pub fn now_modify(mut self) -> Self {
        self.modify_time = Some(bson::DateTime::from_millis(
            Local::now().naive_local().timestamp_millis(),
        ));
        self
    }
}
