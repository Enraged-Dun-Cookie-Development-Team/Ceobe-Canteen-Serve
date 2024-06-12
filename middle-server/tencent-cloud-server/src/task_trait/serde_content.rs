use std::{convert::Infallible, io::Write};

use mime::Mime;
use serde::{Serialize, Serializer};

use crate::error::TcCloudError;

/// 按照特定方法进行请求荷载序列化
pub trait SerializeContentTrait {
    /// 序列化异常
    type Error;

    /// 执行序列化，将结果写入W
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), Self::Error>;

    fn serialize_to<V: Default + Write>(&self) -> Result<V, Self::Error> {
        let mut buff = V::default();
        SerializeContentTrait::serialize(self, &mut buff)?;
        Ok(buff)
    }

    /// 序列化内容的类型
    fn content_type(&self) -> Mime { mime::APPLICATION_OCTET_STREAM }
}

/// 空荷载，不会写入任何内容
pub struct Empty;

impl Serialize for Empty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_none()
    }
}

impl SerializeContentTrait for Empty {
    type Error = Infallible;

    fn serialize<W: Write>(&self, _: &mut W) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// json荷载，会将T序列化为Json并写入W
pub struct Json<'t,T>(pub &'t T);

impl<'t,T: Serialize> SerializeContentTrait for Json<'t,T> {
    type Error = serde_json::Error;

    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), Self::Error> {
        serde_json::to_writer(writer, &self.0)
    }

    fn content_type(&self) -> Mime {
        "application/json; charset=utf-8".parse().unwrap()
    }
}
