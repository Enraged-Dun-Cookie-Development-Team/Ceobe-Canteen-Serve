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
        none(name = "ModifyAt",extra(
            derive(serde::Serialize,serde::Deserialize)
        )),
    )]
    pub ModelMansion{
        /// create record
        #[sub_model(
                want( "ModifyAt")
        )]
        create_time:bson::DateTime
        /// modify time
        #[sub_model(
                want( "ModifyAt")
         )]
        modify_time:Option<bson::DateTime>

        //old fields
        id:MansionId
        description:String
        cvlink:String
        fraction:u8
        daily:Vec<Daily>
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
        Self {
            create_time: now,
            modify_time: None,
        }
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
#[sub_model(
    none(
        // SubModel 可见性
        vis = "",
        // SubModel 名称
        name = "AOnly", 
        // SubModel 额外挂载
        extra(
            doc = "只有A的类型",
            derive(Clone, Copy)
        )
    ),
    // 只有名称可以这样
    all("Copy")
)]
pub struct Value {
    #[sub_model(
        want(
            // 目标SubModel
            for = "AOnly",
            // 字段名称映射
            rename = "good",
            // 指端可见性
            vis = "pub(super)",
            // 字段挂载
            extra(doc = "yes", doc = "CCC")
        ),
        // 只提供目标SubModel 可以这样
        ignore("Copy")
    )]
    a: u32,
    #[sub_model(having(
        for = "Copy",
        rename = "cca",
        extra(doc = "只有b\n  ", doc = "也许不错")
    ))]
    b: String,
}

fn _a() {
    let v = Value {
        a: 11,
        b: String::from("1123"),
    };

    let acc: AOnly = v.into();

    let _c = &acc.good;

    let v = Value {
        a: 11,
        b: String::from("1123"),
    };

    let t: Copy = v.into();
    let _e = &t.cca;
}
