use serde::{
    de::{self, Expected, Unexpected},
    Deserialize, Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    mob_id: String,
    tags: Vec<String>,
    alias: String,
    mobile: String,
    open_push: OpenPush,
    status: Status,
}

/// 是否开启推送
#[derive(Debug, Serialize)]
pub enum OpenPush {
    /// 关闭
    Close = 0,
    /// 开启
    Open = 1,
}

impl<'de> Deserialize<'de> for OpenPush {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let ret = u8::deserialize(deserializer)?;

        match ret {
            0 => Ok(OpenPush::Close),
            1 => Ok(OpenPush::Open),

            num => Err(de::Error::invalid_value(
                Unexpected::Unsigned(num as _),
                &"1u8 or 0u8" as &dyn Expected,
            )),
        }
    }
}

/// 设备状态
#[derive(Debug, Serialize)]
pub enum Status {
    /// 正常
    Fine = 1,
    /// 卸载
    Uninstalled = 3,
    /// 删除
    Delete = 0,
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let ret = u8::deserialize(deserializer)?;

        match ret {
            0 => Ok(Status::Delete),
            1 => Ok(Status::Fine),
            3 => Ok(Status::Uninstalled),

            num => Err(de::Error::invalid_value(
                Unexpected::Unsigned(num as _),
                &"3u8 or 1u8 or 0u8" as &dyn Expected,
            )),
        }
    }
}
