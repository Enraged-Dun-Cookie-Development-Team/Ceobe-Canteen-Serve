use chrono::Local;
use mongodb::bson;

use super::{Daily, Mansion, MansionId};

crate::quick_struct! {
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

impl Default for ModifyAt {
    fn default() -> Self {
        Self {
            create_time: bson::DateTime::from_millis(Local::now().naive_local().timestamp_millis()),
            modify_time: None,
        }
    }
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

impl ModifyAt {
    pub(in crate::serves::mansion) fn now_modify(mut self) -> Self {
        self.modify_time = Some(bson::DateTime::from_millis(
            Local::now().naive_local().timestamp_millis(),
        ));
        self
    }
}
