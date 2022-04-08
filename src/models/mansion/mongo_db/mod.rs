use chrono::Local;
use mongodb::bson;
use sub_model_derive::SubModel;

use super::check::{Daily, Mansion};

crate::quick_struct! {
    pub MansionId{
        main_id:u32
        minor_id:u8
    }
    #[derive(SubModel)]
    #[sub_model(
        none("ModifyAt"),
    )]
    pub ModelMansion{
        /// create record
        #[sub_model(
            field(
                want(name = "ModifyAt")
            )
        )]
        create_time:bson::DateTime
        /// modify time
        #[sub_model(
            field(
                want(name = "ModifyAt")
            )
         )]
        modify_time:Option<bson::DateTime>

        //old fields
        id:MansionId
        description:String
        cvlink:String
        fraction:u8
        daily:Vec<Daily>
    }

    // pub ModifyAt{
    //     create_time:bson::DateTime
    //     modify_time:Option<bson::DateTime>
    // }

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
#[allow(dead_code)]
#[derive(SubModel)]
#[sub_model(none("AOnly"), all("Copy"))]
pub struct Value {
    #[sub_model(field(
        want(name = "AOnly", to = "good"),
        ignore(name = "Copy")
    ))]
    a: u32,
    b: String,
}

#[allow(dead_code)]
fn A() {
    let v = Value {
        a: 11,
        b: String::from("1123"),
    };

    let AOnly { good } = v.into();

    let v = Value {
        a: 11,
        b: String::from("1123"),
    };

    let Copy { b } = v.into();
}
